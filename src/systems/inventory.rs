use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components,
    components::map_position::MapPosition,
    entities::{Player, Weapon},
    GameState,
};

#[derive(Debug, Component, Clone, Copy)]
pub struct Carried {
    pub entity: Entity,
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickUpEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_inventory))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_inventory)
                    .with_system(assign_item),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(cleanup_components::<PlayerInventory>),
            );
    }
}

pub fn assign_item(
    mut commands: Commands,
    mut pick_up_events: EventReader<PickUpEvent>,
    weapons: Query<(Entity, With<Weapon>)>,
    carried_weapons: Query<(Entity, &Carried, With<Weapon>)>,
) {
    pick_up_events.iter().for_each(|event| {
        info!("Pick up event");
        // Remove item from map
        commands.entity(event.item).remove::<MapPosition>();
        commands
            .entity(event.item)
            .remove_bundle::<SpriteSheetBundle>();

        // Add to players inventory
        commands.entity(event.item).insert(Carried {
            entity: event.grabber,
        });

        // if item is a weapon remove the current weapon (if there is one)
        if weapons.contains(event.item) {
            if let Some((current_weapon, _, _)) = carried_weapons
                .iter()
                .filter(|(_, c, _)| c.entity == event.grabber)
                .last()
            {
                commands.entity(current_weapon).despawn_recursive();
            }
        }
    });
}

pub struct PickUpEvent {
    pub grabber: Entity,
    pub item: Entity,
}

/// Entity for caching the Inventory of the player
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
