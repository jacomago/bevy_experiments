use bevy::prelude::default;
use bevy_turborand::RngComponent;

use crate::{
    entities::TileType,
    map::{grid_map::base_map::BaseMap, tile_map::TileMap},
};

use super::{MapArchitect, MapBuilder};

pub struct EmptyArchitect {
    num_monsters: usize,
    num_items: usize,
    entity_distance: f32,
}

impl EmptyArchitect {
    pub fn new(num_monsters: usize, num_items: usize, entity_distance: f32) -> Self {
        Self {
            num_monsters,
            num_items,
            entity_distance,
        }
    }
}

impl MapArchitect for EmptyArchitect {
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            ..default()
        };
        mb.fill(TileType::Floor);
        mb.player_start = mb.map.centre();
        mb.winitem_start = mb.find_most_distant();
        mb.monster_spawns = self.entity_spawns(&mb.map.centre(), &mb.map, rng);
        mb.item_spawns = self.entity_spawns(&mb.map.centre(), &mb.map, rng);
        mb
    }

    fn entity_distance(&self) -> f32 {
        self.entity_distance
    }

    fn num_monsters(&self) -> usize {
        self.num_monsters
    }

    fn num_items(&self) -> usize {
        self.num_items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let mut arch = EmptyArchitect::new(50, 20, 10.0);
        let mut rng = RngComponent::new();
        let mb = arch.builder(40, 60, &mut rng);
        println!("{}", mb);
    }
}
