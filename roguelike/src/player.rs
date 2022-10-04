use crate::{
    prelude::{map_position::MapPosition, *},
    Game,
};

const PLAYER_SPRITE_INDEX: usize = 64;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _player: Player,
    pub position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub fn setup(
    commands: &mut Commands,
    texture_atlas_handle: &Handle<TextureAtlas>,
    player_start: IVec2,
) {
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
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite {
                index: PLAYER_SPRITE_INDEX,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    game: Res<Game>,
    mut query: Query<(&mut MapPosition, &mut Transform, With<Player>)>,
) {
    keyboard_input.get_just_pressed().for_each(|k| {
        let (mut map_position, mut transform, _) = query.single_mut();
        let delta = match k {
            KeyCode::Left => IVec2::new(-1, 0),
            KeyCode::Right => IVec2::new(1, 0),
            KeyCode::Up => IVec2::new(0, 1),
            KeyCode::Down => IVec2::new(0, -1),
            _ => IVec2::ZERO,
        };
        let new_position = map_position.position + delta;
        if game.map.can_enter_tile(new_position) {
            map_position.position = new_position;
            *transform = Transform {
                translation: Vec3::new(
                    (map_position.position.x * TILE_SIZE) as f32,
                    (map_position.position.y * TILE_SIZE) as f32,
                    PLAYER_Z,
                ),
                ..default()
            };
        }
    });
}
