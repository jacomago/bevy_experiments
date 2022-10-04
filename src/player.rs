use crate::actions::Actions;
use crate::loading::TextureAtlasAssets;
use crate::map::map_position::MapPosition;
use crate::{GameState, TILE_SIZE};

use bevy::prelude::*;

const PLAYER_SPRITE_INDEX: usize = 64;
const PLAYER_Z: f32 = 1;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _player: Player,
    pub position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_player));
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAtlasAssets>, player_start: IVec2) {
    commands.spawn_bundle(PlayerBundle {
        position: MapPosition {
            position: player_start,
        },
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(
                    (player_start.x * TILE_SIZE) as f32,
                    (player_start.y * TILE_SIZE) as f32,
                    PLAYER_Z,
                ),
                ..default()
            },
            texture_atlas: textures.texture_atlas,
            sprite: TextureAtlasSprite {
                index: PLAYER_SPRITE_INDEX,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
