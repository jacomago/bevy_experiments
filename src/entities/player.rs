use crate::cleanup::cleanup_components;
use crate::components::health::Health;
use crate::components::map_position::MapPosition;
use crate::config::Settings;
use crate::loading::TextureAtlasAssets;
use crate::map::map_builder::MapBuilder;
use crate::stages::{end_turn, GameStage, TurnState};
use crate::systems::combat::combat;
use crate::systems::fov::{fov, set_fov_visibility, FieldOfView};

use crate::systems::movement::movement;
use crate::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _player: Player,
    pub position: MapPosition,
    pub health: Health,
    pub fov: FieldOfView,
    #[bundle]
    sprite: SpriteSheetBundle,
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
                    .with_system(combat)
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
    fov.update(&player_start, &map_builder.map);
    commands.spawn_bundle(PlayerBundle {
        position: player_start,
        health: Health {
            current: settings.player_settings.max_health,
            max: settings.player_settings.max_health,
        },
        fov: FieldOfView::new(8),
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: player_start
                    .translation(settings.monsters_settings.z_level, settings.tile_size),
                ..default()
            },
            texture_atlas: textures.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: settings.player_settings.entity.sprite_index,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}
