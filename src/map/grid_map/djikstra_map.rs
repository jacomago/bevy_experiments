use ndarray::{Array, Ix2};

use crate::components::map_position::MapPosition;

use super::{base_map::BaseMap, djikstra::DjikstraMapCalc};

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

    pub fn next_along_path(&self, p: &MapPosition) -> MapPosition {
        *self
            .neighbours(p)
            .iter()
            .min_by(|n1, n2| self.value(n1).cmp(&self.value(n2)))
            .unwrap_or(p)
    }

    pub fn furthest_point(&self) -> MapPosition {
        let max = self
            .result
            .indexed_iter()
            .max_by(|x, y| x.1.cmp(y.1))
            .unwrap()
            .0;
        MapPosition::new(max.1 as i32, max.0 as i32)
    }

    pub fn far_points(&self, distance: i32) -> Vec<MapPosition> {
        self.result
            .indexed_iter()
            .filter(|(_, &v)| v.is_none() || v.map(|x| x > distance).unwrap())
            .map(|(p, _)| MapPosition::from_utuple(&p))
            .collect()
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

impl BaseMap for DjikstraMap {
    type Output = Option<i32>;
    fn can_enter_tile(&self, p: &MapPosition) -> bool {
        self.result.get(p.as_utuple()).unwrap_or(&None).is_some()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn value(&self, p: &MapPosition) -> Option<i32> {
        *self.result.get(p.as_utuple()).unwrap_or(&None)
    }

    fn set(&mut self, p: &MapPosition, value: Option<i32>) {
        self.result[p.as_utuple()] = value;
    }
}
impl DjikstraMapCalc for DjikstraMap {}

#[cfg(test)]
mod tests {

    use super::*;
    pub struct TestMap {
        height: usize,
        width: usize,
        result: Array<i32, Ix2>,
    }

    impl TestMap {
        pub fn new(height: usize, width: usize) -> Self {
            Self {
                height,
                width,
                result: Array::<i32, Ix2>::from_elem((height, width), 0),
            }
        }
    }

    impl BaseMap for TestMap {
        type Output = i32;
        fn height(&self) -> usize {
            self.height
        }

        fn width(&self) -> usize {
            self.width
        }
        fn can_enter_tile(&self, p: &MapPosition) -> bool {
            self.result.get(p.as_utuple()).is_some()
        }

        fn value(&self, p: &MapPosition) -> Self::Output {
            self.result[p.as_utuple()]
        }
        fn set(&mut self, p: &MapPosition, value: Self::Output) {
            self.result[p.as_utuple()] = value;
        }
    }

    impl DjikstraMapCalc for TestMap {}

    #[test]
    fn test_next_along_path() {
        let map = TestMap::new(10, 1);
        let dmap = map.djikstra_map(&MapPosition::new(0, 0));
        let next = dmap.next_along_path(&MapPosition::new(0, 1));
        assert_eq!(next, MapPosition::new(0, 0));
    }

    #[test]
    fn test_furthest_point() {
        let map = TestMap::new(10, 1);
        let dmap = map.djikstra_map(&MapPosition::new(0, 0));
        let furthest = dmap.furthest_point();
        assert_eq!(furthest, MapPosition::new(0, 9));
    }

    #[test]
    fn test_path_to() {
        let map = TestMap::new(10, 1);
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
    #[test]
    fn test_calculate_longest_path() {
        let map = TestMap::new(10, 1);
        let dmap = map.djikstra_map(&MapPosition::new(0, 0));
        let path = dmap.calculate_longest_path();
        assert_eq!(
            path,
            (0..10)
                .map(|v| MapPosition::new(0, v))
                .collect::<Vec<MapPosition>>()
        );
    }
}
