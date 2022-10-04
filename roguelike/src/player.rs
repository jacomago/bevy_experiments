use crate::{prelude::*, Game};

const PLAYER_SPRITE_INDEX: usize = 1;

#[derive(Default)]
pub struct Player {
    pub entity: Option<Entity>,
    pub position: IVec2,
}

impl Player {
    pub fn setup(
        &mut self,
        commands: &mut Commands,
        texture_atlas_handle: &Handle<TextureAtlas>,
        map_builder: &MapBuilder,
    ) {
        self.position = map_builder.player_start;
        self.entity = Some(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            (self.position.x * TILE_SIZE) as f32,
                            (self.position.y * TILE_SIZE) as f32,
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
                })
                .id(),
        );
    }
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut transforms: Query<&mut Transform>,
) {
    keyboard_input.get_just_pressed().for_each(|k| {
        let delta = match k {
            KeyCode::Left => IVec2::new(-1, 0),
            KeyCode::Right => IVec2::new(1, 0),
            KeyCode::Up => IVec2::new(0, 1),
            KeyCode::Down => IVec2::new(0, -1),
            _ => IVec2::ZERO,
        };
        let new_position = game.player.position + delta;
        if game.map.can_enter_tile(new_position) {
            game.player.position = new_position;
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(
                    (game.player.position.x * TILE_SIZE) as f32,
                    (game.player.position.y * TILE_SIZE) as f32,
                    PLAYER_Z,
                ),
                ..default()
            };
        }
    });
}
