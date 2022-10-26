use bevy::prelude::*;

use self::{
    combat::CombatPlugin, inventory::InventoryPlugin, movement::MovementPlugin,
    player_input::PlayerInputPlugin,
};

pub mod chasing_player;
pub mod combat;
pub mod fov;
pub mod inventory;
pub mod movement;
pub mod player_input;
pub mod random_actor;

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CombatPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(InventoryPlugin)
            .add_plugin(PlayerInputPlugin);
    }
}
