use ndarray::{Array, Ix2};

use crate::map::map_position::MapPosition;

use super::{djikstra::DjikstraMapCalc, neighbours::Neighbours};

#[derive(Debug)]
pub struct DjikstraMap {
    pub height: usize,
    pub width: usize,
    pub start: (usize, usize),
    result: Array<Option<i32>, Ix2>,
}

impl DjikstraMap {
    pub fn new(height: usize, width: usize, start: (usize, usize)) -> Self {
        let mut result = Array::<Option<i32>, Ix2>::from_elem((height, width), None);
        result[start] = Some(0);
        Self {
            height,
            width,
            start,
            result,
        }
    }

    pub fn value(&self, p: &MapPosition) -> Option<i32> {
        *self.result.get(p.as_utuple()).unwrap_or(&None)
    }

    pub fn set(&mut self, p: &MapPosition, value: i32) {
        self.result[p.as_utuple()] = Some(value);
    }

    pub fn next_along_path(&self, p: &MapPosition) -> MapPosition {
        *self
            .neighbours(p)
            .iter()
            .min_by(|n1, n2| self.value(n1).cmp(&self.value(n2)))
            .unwrap_or(p)
    }

    fn furthest_point(&self) -> MapPosition {
        let max = self
            .result
            .indexed_iter()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap()
            .0;
        MapPosition::new(max.0 as i32, max.1 as i32)
    }

    fn path_to(&self, p: &MapPosition) -> Vec<MapPosition> {
        let mut path = vec![p.to_owned()];
        let mut current = *p;
        while current.as_utuple() != self.start {
            let next = self.next_along_path(&current);
            path.push(next);
            current = next;
        }
        path
    }

    pub fn calculate_longest_path(&self) -> Vec<MapPosition> {
        let new_dmap = self.djikstra_map(&self.furthest_point());
        let new_furthest = new_dmap.furthest_point();
        new_dmap.path_to(&new_furthest)
    }
}

impl Neighbours for DjikstraMap {
    fn can_enter_tile(&self, p: &MapPosition) -> bool {
        self.result.get(p.as_utuple()).unwrap_or(&None).is_some()
    }
}

impl DjikstraMapCalc for DjikstraMap {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}
