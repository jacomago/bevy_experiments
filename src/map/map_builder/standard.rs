use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    components::map_position::MapPosition,
    entities::TileType,
    map::{grid_map::DjikstraMapCalc, tile_map::TileMap},
};

use super::{MapArchitect, MapBuilder};

pub const MAX_ROOM_SIZE: usize = 10;

const NUM_ROOMS: usize = 20;
pub struct StandardArchitect {
    max_room_size: usize,
    num_rooms: usize,
}

impl StandardArchitect {
    pub fn new() -> Self {
        Self {
            max_room_size: MAX_ROOM_SIZE,
            num_rooms: NUM_ROOMS,
        }
    }
}
impl MapArchitect for StandardArchitect {
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            rooms: Vec::new(),
            player_start: MapPosition::default(),
            ..default()
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(
            rng.get_mut(),
            width,
            height,
            self.max_room_size,
            self.num_rooms,
        );
        mb.build_corridors(rng);
        mb.monster_spawns = mb
            .rooms
            .iter()
            .skip(1)
            .map(|room| MapPosition::new(room.x() as i32, room.y() as i32))
            .collect();
        let dmap = mb.map.djikstra_map(&MapPosition::new(
            mb.rooms[0].x() as i32,
            mb.rooms[0].y() as i32,
        ));
        let longest_path = dmap.calculate_longest_path();
        mb.player_start = longest_path[0];
        mb.winitem_start = *longest_path.last().unwrap();
        mb
    }
}
