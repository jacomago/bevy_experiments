use crate::cleanup::cleanup_components;
use crate::components::map_position::MapPosition;
use crate::components::name::EntityName;
use crate::config::{NPCSettings, NPCsSettings, Settings};
use crate::game_ui::tooltip::Interactive;
use crate::map::map_builder::MapBuilder;
use crate::map::GEN_MAP_LABEL;
use crate::stages::{end_turn, TurnState};
use crate::systems::fov::{fov, FieldOfView};
use crate::systems::movement::movement;
use crate::systems::quest::Quest;
use crate::systems::random_actor::{random_move, RandomMover};
use crate::GameState;
use crate::{loading::TextureAtlasAssets, stages::GameStage};

use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use iyes_loopless::prelude::{ConditionSet, IntoConditionalSystem};

use super::MapLevel;
use super::RESPAWN_LABEL;

pub struct NPCsPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for NPCsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_npcs))
            .add_system_set_to_stage(
                GameStage::GenerateMonsterMoves,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(random_move)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MoveMonsters,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(movement)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MonsterFOV,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(fov)
                    .with_system(end_turn)
                    .into(),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    spawn_npcs
                        .run_if_resource_equals(TurnState::NextLevel)
                        .label(RESPAWN_LABEL)
                        .after(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    cleanup_components::<Npc>
                        .run_if_resource_equals(TurnState::NextLevel)
                        .before(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Npc>),
            );
    }
}
#[derive(Component, Default)]
pub struct Npc;

#[derive(Bundle, Default)]
pub struct NPCBundle {
    _m: Npc,
    pub name: EntityName,
    pub position: MapPosition,
    pub interactive: Interactive,
    pub fov: FieldOfView,
    pub quest: Quest,
    #[bundle]
    sprite: SpriteSheetBundle,
}

fn spawn_npcs(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    mut rng: ResMut<GlobalRng>,
    settings: Res<Settings>,
    map_level: Query<&MapLevel>,
) {
    let npc_settings = &settings.npcs_settings;
    map_builder.npc_spawns.iter().for_each(|position| {
        let rng_comp = RngComponent::from(&mut rng);
        spawn_npc(
            &mut commands,
            *position,
            &textures,
            rng_comp,
            npc_settings,
            settings.tile_size,
            match map_level.get_single() {
                Ok(res) => res.value,
                Err(_) => 0,
            },
        );
    });
}

fn weights(setting: &&NPCSettings) -> f64 {
    0.01 * setting.proportion
}

fn spawn_npc(
    commands: &mut Commands,
    position: MapPosition,
    textures: &Res<TextureAtlasAssets>,
    mut rng: RngComponent,
    settings: &NPCsSettings,
    tile_size: i32,
    map_level: u32,
) {
    let level_npcs = &settings
        .npcs
        .iter()
        .filter(|s| s.actor.entity.levels.contains(&map_level))
        .collect::<Vec<_>>();
    let config = rng.weighted_sample(level_npcs, weights).unwrap();
    let mut npc = commands.spawn_bundle(NPCBundle {
        name: EntityName(config.actor.entity.name.clone()),
        position,
        interactive: Interactive {
            text: format!(
                "{} hp:{}",
                &config.actor.entity.name, config.actor.max_health
            ),
        },
        fov: FieldOfView::new(config.actor.fov_radius),
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: position.translation(settings.z_level, tile_size),
                ..default()
            },
            texture_atlas: textures.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: config.actor.entity.sprite_index,
                ..default()
            },

            ..default()
        },
        ..default()
    });
    npc.insert(RandomMover { rng });
    npc.insert(Quest {
        giver: Some(npc.id()),
        requested_item: config.quest_item_type,
    });
}
