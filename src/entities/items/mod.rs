use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components,
    components::{map_position::MapPosition, name::CharacterName},
    game_ui::tooltip::Interactive,
    GameState,
};

use self::winitem::spawn_wintitem;
mod healing;
mod winitem;

pub use winitem::WinItem;

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
    pub name: CharacterName,
    pub position: MapPosition,
    pub interactive: Interactive,
    #[bundle]
    sprite: SpriteSheetBundle,
}
