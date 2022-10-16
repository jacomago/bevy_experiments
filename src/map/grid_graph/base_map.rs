use super::{neighbours::Neighbours, DjikstraMapCalc};

use ndarray::{Array, Ix2};

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
