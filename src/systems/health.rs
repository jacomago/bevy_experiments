use bevy::prelude::*;

use crate::{map::map_position::MapPosition, monsters::Monster, player::Player};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Health>();
    }
}

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
