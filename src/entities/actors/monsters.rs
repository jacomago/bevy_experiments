use crate::cleanup::cleanup_components;
use crate::components::damage::Damage;
use crate::components::map_position::MapPosition;
use crate::config::{Behaviour, MonsterSettings, MonstersSettings, Settings};
use crate::entities::RESPAWN_LABEL;
use crate::loading::TextureAtlasAssets;
use crate::map::map_builder::MapBuilder;
use crate::map::GEN_MAP_LABEL;
use crate::stages::TurnState;
use crate::systems::chasing_player::ChasingPlayer;
use crate::systems::random_actor::RandomMover;
use crate::GameState;

use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use iyes_loopless::prelude::IntoConditionalSystem;

use super::{ActorBundle, MapLevel};

pub struct MonstersPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_monsters))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    spawn_monsters
                        .run_if_resource_equals(TurnState::NextLevel)
                        .label(RESPAWN_LABEL)
                        .after(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    cleanup_components::<Monster>
                        .run_if_resource_equals(TurnState::NextLevel)
                        .before(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Monster>),
            );
    }
}
#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    _m: Monster,
    pub damage: Damage,
    #[bundle]
    actor: ActorBundle,
}

fn spawn_monsters(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    mut rng: ResMut<GlobalRng>,
    settings: Res<Settings>,
    map_level: Query<&MapLevel>,
) {
    let monster_settings = &settings.monsters_settings;
    map_builder.monster_spawns.iter().for_each(|position| {
        let rng_comp = RngComponent::from(&mut rng);
        spawn_monster(
            &mut commands,
            *position,
            &textures,
            rng_comp,
            monster_settings,
            settings.tile_size,
            settings.entity_z_level,
            match map_level.get_single() {
                Ok(res) => res.value,
                Err(_) => 0,
            },
        );
    });
}

fn weights(setting: &&MonsterSettings) -> f64 {
    0.01 * setting.proportion
}

fn spawn_monster(
    commands: &mut Commands,
    position: MapPosition,
    textures: &Res<TextureAtlasAssets>,
    mut rng: RngComponent,
    settings: &MonstersSettings,
    tile_size: i32,
    z_level: f32,
    map_level: u32,
) {
    let level_monsters = &settings
        .monsters
        .iter()
        .filter(|s| s.actor.entity.levels.contains(&map_level))
        .collect::<Vec<_>>();
    if let Some(config) = rng.weighted_sample(level_monsters, weights) {
        let mut monster = commands.spawn(MonsterBundle {
            actor: ActorBundle::from_settings(
                &config.actor,
                position,
                &textures.texture_atlas,
                z_level,
                tile_size,
            ),
            damage: Damage(config.actor.entity.base_damage.unwrap_or(0)),
            ..default()
        });
        match &config.behaviour {
            Behaviour::Random => monster.insert(RandomMover { rng }),
            Behaviour::Chasing => monster.insert(ChasingPlayer {}),
        };
    }
}
