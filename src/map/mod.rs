pub mod grid_graph;
pub mod map_builder;
pub mod tile_map;

use bevy::prelude::*;

use crate::GameState;

use self::map_builder::insert_mapbuilder;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

pub const MAX_ROOM_SIZE: usize = 10;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(insert_mapbuilder));
    }
}
