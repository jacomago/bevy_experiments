use bevy_turborand::RngComponent;
use nannou_core::prelude::Rect;

use crate::components::map_position::MapPosition;
use crate::entities::TileType;

use self::empty::EmptyArchitect;

use super::grid_map::DjikstraMapCalc;
use super::tile_map::TileMap;

mod automata;
mod empty;
mod standard;

trait MapArchitect {
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder;
}

#[derive(Debug, Default)]
pub struct MapBuilder {
    pub map: TileMap,
    pub monster_spawns: Vec<MapPosition>,
    pub player_start: MapPosition,
    pub winitem_start: MapPosition,
}

impl MapBuilder {
    pub fn new(mut rng: RngComponent, height: usize, width: usize) -> Self {
        let mut architect = EmptyArchitect {};
        architect.builder(height, width, &mut rng)
    }

    fn find_most_distant(&self) -> MapPosition {
        self.map.djikstra_map(&self.player_start).furthest_point()
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
