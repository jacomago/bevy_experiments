use crate::components::map_position::MapPosition;
use crate::config::Architect;
use crate::entities::TileType;
use bevy_turborand::RngComponent;

use self::automata::CellularAutomataArchitect;
use self::empty::EmptyArchitect;
use self::standard::StandardArchitect;

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

fn pick_architect(architect: &Architect) -> Box<dyn MapArchitect> {
    match architect {
        Architect::Empty => Box::new(EmptyArchitect {}),
        Architect::Standard => Box::new(StandardArchitect::new()),
        Architect::Automata => Box::new(CellularAutomataArchitect::new()),
    }
}

impl MapBuilder {
    pub fn new(mut rng: RngComponent, height: usize, width: usize, architect: &Architect) -> Self {
        let mut map_arch = pick_architect(&architect);
        map_arch.builder(height, width, &mut rng)
    }

    fn find_most_distant(&self) -> MapPosition {
        self.map.djikstra_map(&self.player_start).furthest_point()
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
