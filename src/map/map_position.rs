use bevy::prelude::*;

pub const TILE_SIZE: i32 = 32;
#[derive(Component, Default, PartialEq, Eq, Clone, Copy, Debug)]
pub struct MapPosition {
    pub position: IVec2,
}

impl MapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: IVec2::from_array([x, y]),
        }
    }

    pub fn translation(self, z: f32) -> Vec3 {
        Vec3::new(
            (self.position.x * TILE_SIZE) as f32,
            (self.position.y * TILE_SIZE) as f32,
            z,
        )
    }

    pub fn from_ivec2(position: IVec2) -> Self {
        Self { position }
    }

    fn from_vec2(position: Vec2) -> Self {
        Self::new(position.x as i32, position.y as i32)
    }

    pub fn overlaps(&self, other: Vec2) -> bool {
        if *self == Self::from_vec2(other) {
            return true;
        }
        false
    }
}
