use std::usize;

use bevy::prelude::*;
use ndarray::{Array, Ix2};

use crate::{loading::TextureAtlasAssets, GameState};

use super::{
    map_builder::{insert_mapbuilder, MapBuilder},
    map_position::MapPosition,
};

const MAP_Z: f32 = 0.0;
const WALL_SPRITE_INDEX: usize = 35;
const FLOOR_SPRITE_INDEX: usize = 46;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(insert_mapbuilder))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum TileType {
    Wall,
    #[default]
    Floor,
}

#[derive(Default, Debug)]
pub struct TileMap {
    pub tiles: Array<TileType, Ix2>,
}

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

pub fn in_bounds(point: IVec2) -> bool {
    point.x >= 0
        && MAP_WIDTH > point.x.try_into().unwrap()
        && point.y >= 0
        && MAP_HEIGHT > point.y.try_into().unwrap()
}

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
            commands.spawn_bundle(SpriteSheetBundle {
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
            });
        });
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            tiles: Array::<TileType, Ix2>::from_elem((MAP_HEIGHT, MAP_WIDTH), TileType::Floor),
        }
    }

    pub fn can_enter_tile(&self, point: &MapPosition) -> bool {
        in_bounds(point.position)
            && self
                .tiles
                .get((point.position.y as usize, point.position.x as usize))
                .map_or(false, |&s| s == TileType::Floor)
    }
}
