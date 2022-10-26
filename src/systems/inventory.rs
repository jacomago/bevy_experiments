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
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_inventory));
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

#[derive(Component, Default, Debug)]
pub struct PlayerInventory {
    pub key_map: Vec<String>,
    pub counts: HashMap<String, u32>,
    pub is_dirty: bool,
}

pub fn update_inventory(
    player_query: Query<(Entity, With<Player>)>,
    player_items: Query<(&Carried, &EntityName)>,
    mut inventory_query: Query<&mut PlayerInventory>,
) {
    let mut new_inventory = HashMap::new();
    let (player, _) = player_query.single();
    player_items
        .iter()
        .filter(|(c, _)| c.entity == player)
        .for_each(|(_, i)| {
            let current = new_inventory.entry(i.0.clone()).or_insert(0);
            *current += 1;
        });
    let mut inventory = inventory_query.single_mut();
    if new_inventory != inventory.counts {
        inventory.is_dirty = true;

        let mut keys = new_inventory.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        inventory.key_map = keys;
        inventory.counts = new_inventory;
    }
}
