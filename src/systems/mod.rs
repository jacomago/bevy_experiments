use bevy::prelude::*;

use self::{collisions::CollisionsPlugin, movement::MovementPlugin};

pub mod collisions;
pub mod movement;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CollisionsPlugin).add_plugin(MovementPlugin);
    }
}
