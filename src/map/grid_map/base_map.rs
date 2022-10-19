use bevy::math::ivec2;

use crate::components::map_position::MapPosition;

pub trait BaseMap {
    type Output;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn can_enter_tile(&self, p: &MapPosition) -> bool;
    fn value(&self, p: &MapPosition) -> Self::Output;
    fn neighbours(&self, p: &MapPosition) -> Vec<MapPosition> {
        vec![ivec2(-1, 0), ivec2(1, 0), ivec2(0, -1), ivec2(0, 1)]
            .iter()
            .map(|iv| MapPosition::from_ivec2(*iv + p.position))
            .filter(|mp| self.can_enter_tile(mp))
            .collect()
    }
}
