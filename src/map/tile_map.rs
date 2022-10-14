use std::usize;

use bevy::prelude::*;
use ndarray::{Array, Ix2};

use crate::loading::TextureAtlasAssets;

use super::{
    map_builder::MapBuilder, map_position::MapPosition, FLOOR_SPRITE_INDEX, MAP_Z,
    WALL_SPRITE_INDEX,
};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum TileType {
    Wall,
    #[default]
    Floor,
}

#[derive(Default, Debug)]
pub struct TileMap {
    pub height: usize,
    pub width: usize,
    pub tiles: Array<TileType, Ix2>,
}

pub fn in_bounds(point: IVec2, width: usize, height: usize) -> bool {
    point.x >= 0
        && width > point.x.try_into().unwrap()
        && point.y >= 0
        && height > point.y.try_into().unwrap()
}

#[derive(Component)]
pub struct Tile;

pub fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
) {
    map_builder
        .map
        .tiles
        .indexed_iter()
        .for_each(|((y, x), t)| {
            let position = MapPosition::new(x.try_into().unwrap(), y.try_into().unwrap());
            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform: Transform {
                        translation: position.translation(MAP_Z),
                        ..default()
                    },
                    texture_atlas: textures.texture_atlas.clone(),
                    sprite: TextureAtlasSprite {
                        index: match *t {
                            TileType::Floor => FLOOR_SPRITE_INDEX,
                            TileType::Wall => WALL_SPRITE_INDEX,
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(Tile);
        });
}

impl TileMap {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            tiles: Array::<TileType, Ix2>::from_elem((height, width), TileType::Floor),
        }
    }

    pub fn in_bounds(&self, point: IVec2) -> bool {
        in_bounds(point, self.width, self.height)
    }

    pub fn can_enter_tile(&self, point: &MapPosition) -> bool {
        self.in_bounds(point.position)
            && self
                .tiles
                .get(point.as_utuple())
                .map_or(false, |&s| s == TileType::Floor)
    }
}
