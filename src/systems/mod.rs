use bevy::prelude::*;

use self::{collisions::CollisionsPlugin, health::HealthPlugin, movement::MovementPlugin};

pub mod collisions;
pub mod health;
pub mod movement;
pub mod name;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollisionsPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(HealthPlugin);
    }
}
