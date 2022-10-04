use bevy::prelude::*;

use crate::TILE_SIZE;

#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
pub struct MapPosition {
    pub position: IVec2,
}

impl MapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: IVec2::from_array([x, y]),
        }
    }

    pub fn translation(&self, z: f32) -> Vec3 {
        Vec3::new(
            (self.position.x * TILE_SIZE) as f32,
            (self.position.y * TILE_SIZE) as f32,
            z,
        )
    }

    pub fn from_translation(translation: Vec3) -> Self {
        MapPosition::new(
            translation.x as i32 / TILE_SIZE,
            translation.y as i32 / TILE_SIZE,
        )
    }
}
