use bevy::math::ivec2;
use ndarray::{Array, Ix2};

use super::{
    map_position::MapPosition,
    tile_map::{TileMap, MAP_HEIGHT, MAP_WIDTH},
};

struct DjikstraMap {
    result: Array<Option<i32>, Ix2>,
}

impl TileMap {
    fn neighbours(&self, p: &MapPosition) -> Vec<MapPosition> {
        vec![ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1), ivec2(0, 1)]
            .iter()
            .map(|iv| MapPosition::from_ivec2(*iv + p.position))
            .filter(|mp| self.can_enter_tile(mp))
            .collect()
    }

    pub fn djikstra(&self, start_node: MapPosition) -> DjikstraMap {
        let mut dmap = DjikstraMap {
            result: Array::<Option<i32>, Ix2>::from_elem((MAP_HEIGHT, MAP_WIDTH), None),
        };

        let mut frontier = vec![start_node];

        while frontier.is_empty() {
            let mut new_frontier: Vec<MapPosition> = vec![];

            frontier.iter().for_each(|f| {
                self.neighbours(f).iter().for_each(|n| {
                    if dmap
                        .result
                        .get((n.position.x as usize, n.position.y as usize))
                        .is_none()
                    {
                        dmap.result[[n.position.x as usize, n.position.y as usize]] =
                            dmap.result[[f.position.x as usize, f.position.y as usize]];
                        new_frontier.push(*n);
                    }
                });
            });
            frontier = new_frontier;
        }
        return dmap;
    }
}
