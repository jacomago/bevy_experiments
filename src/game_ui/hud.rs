use bevy::prelude::*;

use crate::{cleanup::cleanup_components, GameState};

use super::{
    health_bar::{spawn_health_bar, update_hud_health},
    inventory::{spawn_inventory, update_inventory},
};

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_health_bar)
                .with_system(spawn_inventory),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_hud_health)
                .with_system(update_inventory),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Hud>),
        );
    }
}

#[derive(Component)]
pub struct Hud;
