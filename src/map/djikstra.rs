use bevy::math::ivec2;
use ndarray::{Array, Ix2};

use super::{map_position::MapPosition, tile_map::TileMap};

pub trait Neighbours {
    fn can_enter_tile(&self, p: &MapPosition) -> bool;
    fn neighbours(&self, p: &MapPosition) -> Vec<MapPosition> {
        vec![ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1), ivec2(0, 1)]
            .iter()
            .map(|iv| MapPosition::from_ivec2(*iv + p.position))
            .filter(|mp| self.can_enter_tile(mp))
            .collect()
    }
}

#[derive(Debug)]
pub struct DjikstraMap {
    result: Array<Option<i32>, Ix2>,
}

impl DjikstraMap {
    fn new(height: usize, width: usize, start: (usize, usize)) -> Self {
        let mut result = Array::<Option<i32>, Ix2>::from_elem((height, width), None);
        result[start] = Some(0);
        Self { result }
    }

    fn value(&self, p: &MapPosition) -> i32 {
        self.result.get(p.as_utuple()).unwrap().unwrap()
    }

    pub fn next_along_path(&self, p: &MapPosition) -> MapPosition {
        *self
            .neighbours(p)
            .iter()
            .min_by(|n1, n2| self.value(n1).cmp(&self.value(n2)))
            .unwrap_or(p)
    }
}

impl Neighbours for DjikstraMap {
    fn can_enter_tile(&self, p: &MapPosition) -> bool {
        self.result.get(p.as_utuple()).unwrap_or(&None).is_some()
    }
}

impl TileMap {
    pub fn djikstra_map(&self, start_node: &MapPosition) -> DjikstraMap {
        let mut dmap = DjikstraMap::new(self.height, self.width, start_node.as_utuple());

        let mut frontier: Vec<MapPosition> = vec![*start_node];

        while !frontier.is_empty() {
            let mut new_frontier: Vec<MapPosition> = vec![];

            frontier.iter().for_each(|f| {
                self.neighbours(f).iter().for_each(|n| {
                    if dmap.result.get(n.as_utuple()).unwrap().is_none() {
                        dmap.result[n.as_utuple()] = Some(dmap.result[f.as_utuple()].unwrap() + 1);
                        new_frontier.push(*n);
                    }
                });
            });
            frontier = new_frontier;
        }
        dmap
    }
}

#[test]
fn test_djikstra() {
    let map = TileMap::new(10, 20);

    let start = MapPosition::new(0, 0);
    let dmap = map.djikstra_map(&start);
    assert_eq!(dmap.result[[1, 1]], Some(2));
}
