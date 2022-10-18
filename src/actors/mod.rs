use bevy::prelude::*;

use self::{items::ItemsPlugin, monsters::MonstersPlugin, player::PlayerPlugin};

pub mod components;
mod items;
mod monsters;
mod player;

pub use items::WinItem;
pub use monsters::Monster;
pub use player::Player;
pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MonstersPlugin)
            .add_plugin(ItemsPlugin);
    }
}
