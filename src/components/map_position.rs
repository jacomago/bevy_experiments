use bevy::prelude::*;

#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct MapPosition {
    pub position: IVec2,
}

impl MapPosition {
    pub const ZERO: Self = MapPosition {
        position: IVec2::ZERO,
    };

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: IVec2::from_array([x, y]),
        }
    }

    pub fn from_utuple(pair: &(usize, usize)) -> Self {
        Self::new(pair.1 as i32, pair.0 as i32)
    }

    pub fn translation(self, z: f32, tile_size: i32) -> Vec3 {
        Vec3::new(
            (self.position.x * tile_size) as f32,
            (self.position.y * tile_size) as f32,
            z,
        )
    }

    pub fn from_ivec2(position: IVec2) -> Self {
        Self { position }
    }

    pub fn as_utuple(self) -> (usize, usize) {
        (self.position.y as usize, self.position.x as usize)
    }

    pub fn distance(&self, p: &MapPosition) -> f32 {
        let diff = p.position - self.position;
        ((diff.x.pow(2) + diff.y.pow(2)) as f32).powf(0.5)
    }
}
