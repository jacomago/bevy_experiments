use std::usize;

use ndarray::{Array, Ix2};

use crate::prelude::*;

const WALL_SPRITE_INDEX: usize = 35;
const FLOOR_SPRITE_INDEX: usize = 46;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum TileType {
    Wall,
    #[default]
    Floor,
}

#[derive(Default)]
pub struct TileMap {
    pub tiles: Array<TileType, Ix2>,
}

pub fn in_bounds(point: IVec2) -> bool {
    point.x >= 0
        && SCREEN_WIDTH > point.x.try_into().unwrap()
        && point.y >= 0
        && SCREEN_HEIGHT > point.y.try_into().unwrap()
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            tiles: Array::<TileType, Ix2>::from_elem(
                (SCREEN_HEIGHT, SCREEN_WIDTH),
                TileType::Floor,
            ),
        }
    }

    pub fn can_enter_tile(&self, point: IVec2) -> bool {
        in_bounds(point)
            && self
                .tiles
                .get((point.y as usize, point.x as usize))
                .map(|&s| s == TileType::Floor)
                .unwrap_or(false)
    }

    pub fn setup(&self, commands: &mut Commands, texture_atlas_handle: &Handle<TextureAtlas>) {
        self.tiles.indexed_iter().for_each(|((y, x), t)| {
            commands.spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (x as i32 * TILE_SIZE) as f32,
                        (y as i32 * TILE_SIZE) as f32,
                        MAP_Z,
                    ),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: match *t {
                        TileType::Floor => FLOOR_SPRITE_INDEX,
                        TileType::Wall => WALL_SPRITE_INDEX,
                    },
                    ..default()
                },
                ..default()
            });
        });
    }
}