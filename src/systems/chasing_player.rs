use bevy::prelude::*;

use crate::{
    actors::Player,
    map::{map_builder::MapBuilder, map_position::MapPosition},
};

use super::{combat::WantsToAttack, movement::WantsToMove};

#[derive(Component, Default)]
pub struct ChasingPlayer {}

pub fn chase_player(
    player_query: Query<(Entity, &MapPosition, With<Player>)>,
    map: Res<MapBuilder>,
    all_positions: Query<&MapPosition, Without<Player>>,
    mut chasers: Query<(Entity, &mut ChasingPlayer, &MapPosition)>,
    mut move_events: EventWriter<WantsToMove>,
    mut combat_events: EventWriter<WantsToAttack>,
) {
    let (player, player_position, _) = player_query.single();
    let dmap = map.map.djikstra_map(player_position);
    // Find all the new positions
    chasers.iter_mut().for_each(|(entity, _, p)| {
        let destination = dmap.next(p);

        if destination == *player_position {
            info!("Attacking Player");
            combat_events.send(WantsToAttack {
                attacker: entity,
                victim: player,
            });
        } else if !all_positions
            .iter()
            .any(|entity_position| destination == *entity_position)
        {
            move_events.send(WantsToMove {
                entity,
                destination,
            });
        }
    });
}
