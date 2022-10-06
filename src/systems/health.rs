use bevy::prelude::*;

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
