use bevy::{prelude::*, utils::HashMap};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    cleanup::cleanup_components,
    components::{health::Health, map_position::MapPosition},
    config::{ItemSettings, ItemType, ItemsSettings, Settings},
    loading::TextureAtlasAssets,
    map::{map_builder::MapBuilder, GEN_MAP_LABEL},
    stages::TurnState,
    GameState,
};

use self::{dungeonmap::ProvidesMap, healing::ProvidesHealing, winitem::spawn_wintitem};
mod dungeonmap;
mod healing;
mod winitem;

pub use winitem::WinItem;

use super::{GameEntityBundle, MapLevel, RESPAWN_LABEL};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_wintitem)
                .with_system(spawn_items),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Item>),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(
                    spawn_wintitem
                        .run_if_resource_equals(TurnState::NextLevel)
                        .label(RESPAWN_LABEL)
                        .after(GEN_MAP_LABEL),
                )
                .with_system(
                    spawn_items
                        .run_if_resource_equals(TurnState::NextLevel)
                        .label(RESPAWN_LABEL)
                        .after(GEN_MAP_LABEL),
                ),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(
                cleanup_components::<Item>
                    .run_if_resource_equals(TurnState::NextLevel)
                    .before(GEN_MAP_LABEL),
            ),
        )
        .add_event::<ActivateItem>();
    }
}

#[derive(Component, Default)]
pub struct Item;

#[derive(Bundle, Default)]
pub struct ItemBundle {
    _i: Item,
    #[bundle]
    pub entity: GameEntityBundle,
}

fn spawn_items(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    mut rng: ResMut<GlobalRng>,
    settings: Res<Settings>,
    map_level: Query<&MapLevel>,
) {
    let item_settings = &settings.items_settings;
    map_builder.item_spawns.iter().for_each(|position| {
        let rng_comp = RngComponent::from(&mut rng);
        spawn_item(
            &mut commands,
            *position,
            &textures,
            rng_comp,
            item_settings,
            settings.tile_size,
            match map_level.get_single() {
                Ok(res) => res.value,
                Err(_) => 0,
            },
        );
    });
}

fn weights(setting: &&ItemSettings) -> f64 {
    0.01 * setting.proportion
}

fn spawn_item(
    commands: &mut Commands,
    position: MapPosition,
    textures: &Res<TextureAtlasAssets>,
    mut rng: RngComponent,
    settings: &ItemsSettings,
    tile_size: i32,
    map_level: u32,
) {
    let level_items = &settings
        .items
        .iter()
        .filter(|s| s.entity.levels.contains(&map_level))
        .collect::<Vec<_>>();
    let config = rng.weighted_sample(level_items, weights).unwrap();
    let mut item = commands.spawn_bundle(ItemBundle {
        entity: GameEntityBundle::from_settings(
            &config.entity,
            position,
            &textures.texture_atlas,
            settings.z_level,
            tile_size,
        ),
        ..default()
    });
    match &config.item_type {
        ItemType::Healing => item.insert(ProvidesHealing {
            amount: config.effect_amount.unwrap(),
        }),
        ItemType::DungeonMap => item.insert(ProvidesMap),
    };
}

#[derive(Debug)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

pub fn activate(
    mut commands: Commands,
    mut activation_events: EventReader<ActivateItem>,
    mut healths: Query<&mut Health>,
    provides_healing: Query<&ProvidesHealing>,
    provides_map: Query<&ProvidesMap>,
    items: Query<&Item>,
    mut visibility_query: Query<(&mut Visibility, With<MapPosition>)>,
) {
    let mut to_heal = HashMap::new();
    activation_events.iter().for_each(|event| {
        if items.contains(event.item) {
            // healing
            if let Ok(healing) = provides_healing.get(event.item) {
                info!("used healing");
                to_heal
                    .entry(event.used_by)
                    .and_modify(|current_heal| *current_heal += healing.amount)
                    .or_insert(healing.amount);
            }

            // reveal map
            if provides_map.get(event.item).is_ok() {
                info!("reveal map");
                visibility_query.iter_mut().for_each(|(mut visibility, _)| {
                    visibility.is_visible = true;
                });
            }
            commands.entity(event.item).despawn_recursive();
        }
    });

    to_heal.iter().for_each(|(entity, heal_amount)| {
        if let Ok(mut health) = healths.get_mut(*entity) {
            health.current = health.max.min(health.current + heal_amount);
        }
    });
}
