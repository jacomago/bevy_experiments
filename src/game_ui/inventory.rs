use bevy::prelude::*;

use crate::{
    components::name::EntityName,
    systems::inventory::{Carried, PlayerInventory},
};

use super::hud::UiState;

pub fn update_inventory_hud(
    mut inventory_query: Query<&mut PlayerInventory>,
    items: Query<(With<Carried>, &EntityName)>,
    mut ui_status: ResMut<UiState>,
) {
    let mut inventory = inventory_query.single_mut();
    if !inventory.is_dirty {
        return;
    }

    ui_status.inventory = inventory
        .key_map
        .iter()
        .enumerate()
        .map(|(i, entity)| {
            let (_, name) = items.get(*entity).unwrap();
            format!("{}: {}", i, name)
        })
        .collect();

    inventory.is_dirty = false;
}
