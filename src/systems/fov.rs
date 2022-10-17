use bevy::{prelude::*, utils::HashSet};

use crate::map::map_position::MapPosition;

pub struct FOVPlugin;

impl Plugin for FOVPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Default)]
pub struct FieldOfView {
    pub visible_positions: HashSet<MapPosition>,
    pub radius: i32,
    pub is_dirty: bool,
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_positions: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_positions: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
