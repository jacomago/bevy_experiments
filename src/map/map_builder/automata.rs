use bevy::{
    math::ivec2,
    prelude::{default, IVec2},
};
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    components::map_position::MapPosition,
    entities::TileType,
    map::{grid_map::base_map::BaseMap, tile_map::TileMap},
};

use super::{MapArchitect, MapBuilder};

pub struct CellularAutomataArchitect {
    percent_floor: i32,
    max_neighbours: usize,
    min_neighbours: usize,
}

impl MapArchitect for CellularAutomataArchitect {
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
        self.random_noise_map(rng, &mut mb.map);
        self.iteration(&mut mb.map);
        mb
    }
}
impl CellularAutomataArchitect {
    fn new() -> Self {
        Self {
            percent_floor: 45,
            max_neighbours: 4,
            min_neighbours: 0,
        }
    }

    fn find_start(&self, map: &TileMap) -> MapPosition {
        let center = MapPosition::new(map.width as i32 / 2, map.height as i32 / 2);
        let closest_point = map
            .tiles
            .indexed_iter()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                let pos = MapPosition::new(idx.1 as i32, idx.0 as i32);
                (pos, center.distance(&pos))
            })
            .min_by(|(_, d), (_, d2)| d.partial_cmp(d2).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        closest_point
    }

    fn random_noise_map(&mut self, rng: &mut bevy_turborand::RngComponent, map: &mut TileMap) {
        map.tiles.iter_mut().for_each(|tile| {
            let roll = rng.i32(0..100);
            if roll < self.percent_floor {
                *tile = TileType::Floor;
            } else {
                *tile = TileType::Wall;
            }
        });
    }

    fn count_neighbours(&self, v: IVec2, map: &TileMap) -> usize {
        let mut count = 0;
        (-1..1).for_each(|xi| {
            (-1..1).for_each(|yi| {
                if !(xi == 0 && yi == 0) && map.value(&MapPosition::from_ivec2(v)) == TileType::Wall
                {
                    count += 1
                }
            })
        });
        count
    }
    fn iteration(&mut self, map: &mut TileMap) {
        let mut new_tiles = map.tiles.clone();
        (1..map.height - 1).for_each(|y| {
            (1..map.width - 1).for_each(|x| {
                let neighbours = self.count_neighbours(ivec2(x as i32, y as i32), map);
                if neighbours > self.max_neighbours || neighbours == self.min_neighbours {
                    new_tiles[(y, x)] = TileType::Wall;
                } else {
                    new_tiles[(y, x)] = TileType::Floor;
                }
            })
        });
        map.tiles = new_tiles;
    }
}

#[test]
fn build() {
    let mut arch = CellularAutomataArchitect::new();
    let mut rng = RngComponent::new();
    let mb = arch.builder(40, 80, &mut rng);
    println!("{}", mb.map);
}
