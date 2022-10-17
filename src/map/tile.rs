use bevy::prelude::*;

use crate::loading::TextureAtlasAssets;

use super::{map_position::MapPosition, FLOOR_SPRITE_INDEX, MAP_Z, WALL_SPRITE_INDEX};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, Component)]
pub enum TileType {
    Wall,
    #[default]
    Floor,
}

#[derive(Component, Default)]
pub struct Tile;

#[derive(Bundle, Default)]

pub struct TileBundle {
    _t: Tile,
    tile_type: TileType,
    position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl TileBundle {
    pub fn new(position: MapPosition, textures: &TextureAtlasAssets, tile_type: TileType) -> Self {
        Self {
            position,
            tile_type,
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position.translation(MAP_Z),
                    ..default()
                },
                texture_atlas: textures.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: match tile_type {
                        TileType::Floor => FLOOR_SPRITE_INDEX,
                        TileType::Wall => WALL_SPRITE_INDEX,
                    },
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}
