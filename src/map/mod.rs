pub mod grid_graph;
pub mod map_builder;
pub mod map_position;
pub mod tile;
pub mod tile_map;

use bevy::prelude::*;

use crate::{cleanup::cleanup_components, GameState};

use self::{map_builder::insert_mapbuilder, tile::Tile, tile_map::spawn_map};

const MAP_Z: f32 = 0.0;
const WALL_SPRITE_INDEX: usize = 35;
const FLOOR_SPRITE_INDEX: usize = 46;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

pub const MAX_ROOM_SIZE: usize = 10;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(insert_mapbuilder))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Tile>),
            );
    }
}
