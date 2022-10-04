use nannou_core::prelude::Rect;

use crate::prelude::{map_position::MapPosition, *};

const MONSTER_SPRITE_INDEX: usize = 69;

#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    _m: Monster,
    pub position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub fn collisions(
    mut commands: Commands,
    monsters: Query<(Entity, &MapPosition, With<Monster>)>,
    player_position_query: Query<&MapPosition, With<Player>>,
) {
    let player_position = player_position_query.single();
    monsters
        .iter()
        .filter(|(_, p, _)| *p == player_position)
        .for_each(|(e, _, _)| commands.entity(e).despawn_recursive());
}

pub fn setup(commands: &mut Commands, texture_atlas_handle: &Handle<TextureAtlas>, rooms: &[Rect]) {
    rooms.iter().skip(1).for_each(|room| {
        let position = bevy::prelude::IVec2::from_array(room.xy().as_i32().to_array());
        commands.spawn_bundle(MonsterBundle {
            position: MapPosition { position },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (position.x * TILE_SIZE) as f32,
                        (position.y * TILE_SIZE) as f32,
                        MONSTER_Z,
                    ),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: MONSTER_SPRITE_INDEX,
                    ..default()
                },
                ..default()
            },
            ..default()
        });
    });
}
