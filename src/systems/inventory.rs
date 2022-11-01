use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components,
    components::map_position::MapPosition,
    config::ItemType,
    entities::{FetchItem, Player, ProvidesHealing, ProvidesMap, QuestState, Weapon},
    GameState,
};

use super::quest_engine::AssignedQuest;

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
    healing: Query<(Entity, With<ProvidesHealing>)>,
    dungeon_maps: Query<(Entity, With<ProvidesMap>)>,
    carried_weapons: Query<(Entity, &Carried, With<Weapon>)>,
    mut assigned_fetch_quests: Query<(&FetchItem, &mut QuestState), With<AssignedQuest>>,
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

        // Find the item type
        let current_item_type = if healing.contains(event.item) {
            ItemType::Healing
        } else if dungeon_maps.contains(event.item) {
            ItemType::DungeonMap
        } else {
            ItemType::Weapon
        };

        // If quest exists on new holder of item for quest, mark quest as updated
        // TODO decide if all quests should be marked as updated, or just one
        // TODO decide if future quests should be marked as updated
        assigned_fetch_quests
            .iter_mut()
            .filter(|(q, _)| q.requested_item == current_item_type)
            .for_each(|(_, mut q)| {
                info!("update the quest");
                *q = QuestState::Updated;
            });
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
