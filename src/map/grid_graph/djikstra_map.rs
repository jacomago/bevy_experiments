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

#[cfg(test)]
mod tests {

    use super::*;
    pub struct BaseMap {
        height: usize,
        width: usize,
        result: Array<i32, Ix2>,
    }

    impl BaseMap {
        pub fn new(height: usize, width: usize) -> Self {
            Self {
                height,
                width,
                result: Array::<i32, Ix2>::from_elem((height, width), 0),
            }
        }
    }

    impl Neighbours for BaseMap {
        fn can_enter_tile(&self, p: &crate::map::map_position::MapPosition) -> bool {
            self.result.get(p.as_utuple()).is_some()
        }
    }

    impl DjikstraMapCalc for BaseMap {
        fn height(&self) -> usize {
            self.height
        }

        fn width(&self) -> usize {
            self.width
        }
    }

    #[test]
    fn test_next_along_path() {
        let map = BaseMap::new(10, 1);
        let dmap = map.djikstra_map(&MapPosition::new(0, 0));
        let next = dmap.next_along_path(&MapPosition::new(0, 1));
        assert_eq!(next, MapPosition::new(0, 0));
    }

    #[test]
    fn test_furthest_point() {
        let map = BaseMap::new(10, 1);
        let dmap = map.djikstra_map(&MapPosition::new(0, 0));
        let furthest = dmap.furthest_point();
        assert_eq!(furthest, MapPosition::new(9, 0));
    }

    #[test]
    fn test_path_to() {
        let map = BaseMap::new(10, 1);
        let dmap = map.djikstra_map(&MapPosition::new(0, 0));
        let path = dmap.path_to(&MapPosition::new(0, 2));
        assert_eq!(
            path,
            vec![
                MapPosition::new(0, 2),
                MapPosition::new(0, 1),
                MapPosition::new(0, 0)
            ]
        );
    }
}
