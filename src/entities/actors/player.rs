use crate::cleanup::cleanup_components;
use crate::components::damage::Damage;
use crate::components::map_position::MapPosition;
use crate::config::Settings;
use crate::entities::items::activate;
use crate::entities::RESPAWN_LABEL;
use crate::loading::TextureAtlasAssets;
use crate::map::map_builder::MapBuilder;
use crate::map::GEN_MAP_LABEL;
use crate::stages::{end_turn, GameStage, TurnState};
use crate::systems::combat::combat;
use crate::systems::fov::{fov, set_fov_visibility, FieldOfView};

use crate::systems::movement::movement;
use crate::systems::quest_engine::interact_quest_giver;
use crate::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use super::ActorBundle;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Default)]
pub struct MapLevel {
    pub value: u32,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _player: Player,
    pub level: MapLevel,
    pub damage: Damage,
    #[bundle]
    actor: ActorBundle,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(set_fov_visibility),
            )
            .add_system_set_to_stage(
                GameStage::PlayerCombat,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::PlayerTurn)
                    .with_system(activate)
                    .with_system(combat)
                    .with_system(interact_quest_giver)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MovePlayer,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::PlayerTurn)
                    .with_system(movement)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::PlayerFOV,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::PlayerTurn)
                    .with_system(fov)
                    .with_system(end_turn)
                    .into(),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    player_next_level
                        .run_if_resource_equals(TurnState::NextLevel)
                        .label(RESPAWN_LABEL)
                        .after(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Player>),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    settings: Res<Settings>,
) {
    let player_start = map_builder.player_start;
    let mut fov = FieldOfView::new(settings.player_settings.fov_radius);
    fov.update(player_start, &map_builder.map);
    commands.spawn_bundle(PlayerBundle {
        damage: Damage(settings.player_settings.entity.base_damage.unwrap_or(0)),
        actor: ActorBundle::from_settings(
            &settings.player_settings,
            player_start,
            &textures.texture_atlas,
            settings.entity_z_level,
            settings.tile_size,
        ),
        ..default()
    });
}

fn player_next_level(
    mut player_location: Query<(
        &mut MapPosition,
        &mut Transform,
        &mut FieldOfView,
        With<Player>,
    )>,
    map_builder: Res<MapBuilder>,
    settings: Res<Settings>,
) {
    let (mut pos, mut trans, mut fov, _) = player_location.single_mut();
    *pos = map_builder.player_start;
    trans.translation = map_builder
        .player_start
        .translation(trans.translation.z, settings.tile_size);
    *fov = FieldOfView::new(settings.player_settings.fov_radius);
    fov.update(*pos, &map_builder.map);
}
