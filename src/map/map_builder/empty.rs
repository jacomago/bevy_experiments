use bevy_turborand::RngComponent;

use crate::{
    components::map_position::MapPosition,
    entities::TileType,
    map::{grid_map::base_map::BaseMap, tile_map::TileMap},
};

use super::{MapArchitect, MapBuilder};

const NUM_MONSTERS: usize = 50;
const MONSTER_DISTANCE: f32 = 10.0;
pub struct EmptyArchitect {
    num_monsters: usize,
    monster_distance: f32,
}

impl EmptyArchitect {
    pub fn new() -> Self {
        Self {
            num_monsters: NUM_MONSTERS,
            monster_distance: MONSTER_DISTANCE,
        }
    }
}

impl MapArchitect for EmptyArchitect {
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            monster_spawns: Vec::new(),
            player_start: MapPosition::ZERO,
            winitem_start: MapPosition::ZERO,
        };
        mb.fill(TileType::Floor);
        mb.player_start = mb.map.centre();
        mb.winitem_start = mb.find_most_distant();
        mb.monster_spawns = self.monster_spawns(&mb.map.centre(), &mb.map, rng);
        mb
    }

    fn monster_distance(&self) -> f32 {
        self.monster_distance
    }

    fn num_monsters(&self) -> usize {
        self.num_monsters
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let mut arch = EmptyArchitect::new();
        let mut rng = RngComponent::new();
        let mb = arch.builder(10, 20, &mut rng);
        println!("{}", mb);
    }
}
