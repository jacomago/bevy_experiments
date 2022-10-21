use bevy::prelude::*;

use crate::{cleanup::cleanup_components, GameState};

use self::winitem::spawn_wintitem;
mod healing;
mod winitem;

pub use winitem::WinItem;

use super::GameEntityBundle;

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_wintitem))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Item>),
            );
    }
}

#[derive(Component, Default)]
pub struct Item;

#[derive(Bundle, Default)]
pub struct ItemBundle {
    _i: Item,
    #[bundle]
    pub entity: GameEntityBundle,
}
