use bevy::prelude::*;

use self::{combat::CombatPlugin, health::HealthPlugin, movement::MovementPlugin};

pub mod combat;
pub mod health;
pub mod movement;
pub mod name;
pub mod random_actor;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CombatPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(HealthPlugin);
    }
}
