use bevy::prelude::*;

use self::{combat::CombatPlugin, movement::MovementPlugin};

pub mod chasing_player;
pub mod combat;
pub mod movement;
pub mod random_actor;
pub mod fov;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CombatPlugin).add_plugin(MovementPlugin);
    }
}
