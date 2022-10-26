use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    actions::Actions,
    components::{health::Health, map_position::MapPosition},
    entities::{ActivateItem, Item, Monster, Player},
    stages::TurnState,
    GameState,
};

use super::{
    combat::WantsToAttack,
    inventory::{Carried, PlayerInventory},
    movement::WantsToMove,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(pick_up.run_if_resource_equals(TurnState::AwaitingInput))
                .with_system(movement.run_if_resource_equals(TurnState::AwaitingInput))
                .with_system(wait.run_if_resource_equals(TurnState::AwaitingInput))
                .with_system(use_item.run_if_resource_equals(TurnState::AwaitingInput)),
        );
    }
}

fn pick_up(
    mut commands: Commands,
    actions: Res<Actions>,
    player_query: Query<(Entity, &MapPosition, With<Player>)>,
    items: Query<(Entity, &MapPosition, With<Item>)>,
) {
    if actions.pick_up_item.is_some() {
        let (entity, position, _) = player_query.single();
        let poss_item = items.iter().filter(|(_, p, _)| position == *p).last();
        if let Some((item_entity, _, _)) = poss_item {
            commands.entity(item_entity).remove::<MapPosition>();
            commands
                .entity(item_entity)
                .remove_bundle::<SpriteSheetBundle>();
            commands.entity(item_entity).insert(Carried { entity });
        }

        commands.insert_resource(TurnState::PlayerTurn);
    }
}

fn movement(
    mut commands: Commands,
    actions: Res<Actions>,
    mut move_events: EventWriter<WantsToMove>,
    mut combat_events: EventWriter<WantsToAttack>,
    player_query: Query<(Entity, &MapPosition, With<Player>)>,
    monsters: Query<(Entity, &MapPosition, With<Monster>)>,
) {
    if actions.player_movement.is_some() {
        let movement = actions.player_movement.unwrap().as_ivec2();

        if movement != IVec2::ZERO {
            let (entity, position, _) = player_query.single();
            let new_position = MapPosition::from_ivec2(position.position + movement);

            let monster = monsters
                .iter()
                .filter(|(_, m, _)| **m == new_position)
                .last();
            if let Some((m, _, _)) = monster {
                combat_events.send(WantsToAttack {
                    attacker: entity,
                    victim: m,
                });
            } else {
                move_events.send(WantsToMove {
                    entity,
                    destination: new_position,
                });
            }

            commands.insert_resource(TurnState::PlayerTurn);
        }
    }
}

fn wait(
    mut commands: Commands,
    actions: Res<Actions>,
    mut player_health: Query<&mut Health, With<Player>>,
) {
    if actions.player_movement.is_some() {
        let movement = actions.player_movement.unwrap().as_ivec2();

        if movement == IVec2::ZERO {
            let mut health = player_health.single_mut();
            health.current = (health.current + 1).min(health.max);
        }

        commands.insert_resource(TurnState::PlayerTurn);
    }
}

fn use_item(
    mut commands: Commands,
    actions: Res<Actions>,
    mut use_events: EventWriter<ActivateItem>,
    inventory_query: Query<&PlayerInventory>,
    player_query: Query<(Entity, With<Player>)>,
) {
    if actions.use_item.is_some() {
        let item_key = actions.use_item.unwrap();
        let inventory = inventory_query.single();
        let (player, _) = player_query.single();

        use_events.send(ActivateItem {
            used_by: player,
            item: inventory.key_map[item_key],
        });

        commands.insert_resource(TurnState::PlayerTurn);
    }
}
