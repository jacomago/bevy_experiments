use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};

use crate::{
    cleanup::cleanup_components,
    components::map_position::MapPosition,
    config::{ItemSettings, ItemType, ItemsSettings, Settings},
    loading::TextureAtlasAssets,
    map::map_builder::MapBuilder,
    GameState,
};

use self::{dungeonmap::ProvidesMap, healing::ProvidesHealing, winitem::spawn_wintitem};
mod dungeonmap;
mod healing;
mod winitem;

pub use winitem::WinItem;

use super::GameEntityBundle;

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
        );
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
) {
    let item_settings = &settings.items_settings;
    map_builder.item_spawns.iter().for_each(|position| {
        let rng_comp = RngComponent::from(&mut rng);
        spawn_item(
            &mut commands,
            position,
            &textures,
            rng_comp,
            item_settings,
            settings.tile_size,
            settings.items_settings.z_level,
        );
    });
}

fn weights(setting: &ItemSettings) -> f64 {
    0.01 * setting.proportion
}

fn spawn_item(
    commands: &mut Commands,
    position: &MapPosition,
    textures: &Res<TextureAtlasAssets>,
    mut rng: RngComponent,
    settings: &ItemsSettings,
    tile_size: i32,
    z_level: f32,
) {
    let config = rng.weighted_sample(&settings.items, weights).unwrap();
    let mut item = commands.spawn_bundle(ItemBundle {
        entity: GameEntityBundle::from_settings(
            &config.entity,
            position,
            &textures.texture_atlas,
            z_level,
            tile_size,
        ),
        ..default()
    });
    match &config.item_type {
        ItemType::Healing => item.insert(ProvidesHealing { amount: 6 }),
        ItemType::DungeonMap => item.insert(ProvidesMap),
    };
}
