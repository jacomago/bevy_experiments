use bevy::prelude::*;

use self::{monsters::MonstersPlugin, player::PlayerPlugin};

pub mod components;
mod monsters;
mod player;

pub use player::Player;
pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin).add_plugin(MonstersPlugin);
    }
}
