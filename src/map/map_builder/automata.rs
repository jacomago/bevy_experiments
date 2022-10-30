use bevy::{
    math::ivec2,
    prelude::{default, IVec2},
};
use bevy_turborand::DelegatedRng;

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
    num_monsters: usize,
    num_items: usize,
    num_npcs: usize,
    entity_distance: f32,
}

impl MapArchitect for CellularAutomataArchitect {
    fn entity_distance(&self) -> f32 {
        self.entity_distance
    }

    fn num_monsters(&self) -> usize {
        self.num_monsters
    }
    fn num_items(&self) -> usize {
        self.num_items
    }
    fn num_npcs(&self) -> usize {
        self.num_npcs
    }

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
        mb.player_start = self.find_start(&mb.map);
        mb.fill_in_unreachable();
        mb.monster_spawns = self.entity_spawns(mb.player_start, &mb.map, rng, self.num_monsters);

        mb.item_spawns = self.entity_spawns(mb.player_start, &mb.map, rng, self.num_items);
        mb.npc_spawns = self.entity_spawns(mb.player_start, &mb.map, rng, self.num_npcs);
        mb.winitem_start = mb.find_most_distant();
        mb
    }
}
impl CellularAutomataArchitect {
    pub fn new(
        num_monsters: usize,
        num_items: usize,
        num_npcs: usize,
        entity_distance: f32,
    ) -> Self {
        Self {
            percent_floor: 45,
            max_neighbours: 4,
            min_neighbours: 0,
            num_items,
            num_monsters,
            num_npcs,
            entity_distance,
        }
    }
    fn find_start(&self, map: &TileMap) -> MapPosition {
        let center = map.centre();
        let closest_point = map
            .tiles
            .indexed_iter()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                let pos = MapPosition::new(idx.1 as i32, idx.0 as i32);
                (pos, center.distance(pos))
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
                if !(xi == 0 && yi == 0) && map.value(MapPosition::from_ivec2(v)) == TileType::Wall
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

#[cfg(test)]
mod tests {
    use bevy_turborand::RngComponent;

    use super::*;

    #[test]
    fn build() {
        let mut arch = CellularAutomataArchitect::new(50, 20, 10, 10.0);
        let mut rng = RngComponent::new();
        let mb = arch.builder(40, 80, &mut rng);
        println!("{}", mb);
    }
}
