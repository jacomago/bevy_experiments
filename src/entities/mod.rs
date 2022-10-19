use bevy::prelude::*;

use self::{items::ItemsPlugin, monsters::MonstersPlugin, player::PlayerPlugin, tile::TilePlugin};

mod items;
mod monsters;
mod player;
mod tile;

pub use items::WinItem;
pub use monsters::Monster;
pub use player::Player;
pub use tile::Tile;
pub use tile::TileType;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MonstersPlugin)
            .add_plugin(ItemsPlugin)
            .add_plugin(TilePlugin);
    }
}
