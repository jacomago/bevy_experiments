use std::collections::HashMap;

use bevy::prelude::*;

use crate::{components::name::EntityName, entities::Player, GameState};

#[derive(Debug, Component, Clone, Copy)]
pub struct Carried {
    pub entity: Entity,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_inventory))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_inventory))
            .add_event::<ActivateItem>();
    }
}

#[derive(Component, Default, Debug)]
pub struct Inventory;

#[derive(Bundle, Debug, Default)]
pub struct InventoryBundle {
    _i: Inventory,
    player: PlayerInventory,
}

fn spawn_inventory(mut commands: Commands) {
    commands.spawn_bundle(InventoryBundle { ..default() });
}

pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Component, Default, Debug)]
pub struct PlayerInventory {
    pub key_map: Vec<Entity>,
    pub is_dirty: bool,
}

pub fn update_inventory(
    player_query: Query<(Entity, With<Player>)>,
    all_items: Query<(Entity, &Carried)>,
    mut inventory_query: Query<&mut PlayerInventory>,
) {
    let mut inventory = inventory_query.single_mut();
    let (player, _) = player_query.single();
    let mut player_items = all_items
        .iter()
        .filter(|(_, c)| c.entity == player)
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    player_items.sort();
    if inventory.key_map != player_items {
        inventory.is_dirty = true;
        inventory.key_map = player_items;
    }
}
