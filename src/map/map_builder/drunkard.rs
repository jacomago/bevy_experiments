use bevy::prelude::default;
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    components::map_position::MapPosition,
    entities::TileType,
    map::{
        grid_map::{base_map::BaseMap, DjikstraMapCalc},
        tile_map::TileMap,
    },
};

use super::{MapArchitect, MapBuilder};

pub struct DrunkardArchitect {
    stagger_distance: usize,
    ratio: f32,
    num_monsters: usize,
    num_items: usize,
    entity_distance: f32,
    max_distance: i32,
}

impl DrunkardArchitect {
    pub fn new(num_monsters: usize, num_items: usize, entity_distance: f32) -> Self {
        Self {
            stagger_distance: 400,
            ratio: 0.3,
            num_monsters,
            num_items,
            entity_distance,
            max_distance: 2000,
        }
    }
    fn drunkard(&mut self, start: MapPosition, rng: &mut RngComponent, map: &mut TileMap) {
        let mut drunkard = start;
        let mut distance_staggered = 0;

        loop {
            map.set(&drunkard, TileType::Floor);

            match rng.usize(0..4) {
                0 => drunkard.position.x -= 1,
                1 => drunkard.position.x += 1,
                2 => drunkard.position.y -= 1,
                _ => drunkard.position.y += 1,
            }
            if !map.in_bounds(&drunkard) {
                break;
            }

            distance_staggered += 1;
            if distance_staggered > self.stagger_distance {
                break;
            }
        }
    }
}

impl MapArchitect for DrunkardArchitect {
    fn builder(
        &mut self,
        height: usize,
        width: usize,
        rng: &mut bevy_turborand::RngComponent,
    ) -> super::MapBuilder {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            ..default()
        };
        mb.fill(TileType::Wall);
        mb.player_start = mb.map.centre();
        self.drunkard(mb.map.centre(), rng, &mut mb.map);
        let num_open_tiles = (self.ratio * (width * height) as f32) as usize;
        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < num_open_tiles
        {
            let rand_pos = MapPosition::new(rng.i32(0..width as i32), rng.i32(0..height as i32));
            self.drunkard(rand_pos, rng, &mut mb.map);
            mb.map
                .djikstra_map(&mb.player_start)
                .far_points(Some(self.max_distance))
                .iter()
                .for_each(|p| {
                    mb.map.set(p, TileType::Wall);
                });
        }
        mb.monster_spawns = self.entity_spawns(&mb.player_start, &mb.map, rng);
        mb.item_spawns = self.entity_spawns(&mb.player_start, &mb.map, rng);
        mb.winitem_start = mb.find_most_distant();
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
        let mut arch = DrunkardArchitect::new(50, 20, 10.0);
        let mut rng = RngComponent::new();
        let mb = arch.builder(40, 80, &mut rng);
        println!("{}", mb);
    }
}
