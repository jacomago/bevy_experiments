use bevy::prelude::*;

use crate::{
    components::map_position::MapPosition,
    entities::{Monster, Player, MONSTER_FOV_RADIUS},
    map::{grid_map::DjikstraMapCalc, map_builder::MapBuilder},
};

use super::{combat::WantsToAttack, fov::FieldOfView, movement::WantsToMove};

#[derive(Component, Default)]
pub struct ChasingPlayer {}

pub fn chase_player(
    player_query: Query<(Entity, &MapPosition, With<Player>)>,
    map: Res<MapBuilder>,
    all_positions: Query<&MapPosition, With<Monster>>,
    mut chasers: Query<(Entity, &mut ChasingPlayer, &FieldOfView, &MapPosition)>,
    mut move_events: EventWriter<WantsToMove>,
    mut combat_events: EventWriter<WantsToAttack>,
) {
    let (player, player_position, _) = player_query.single();
    let dmap = map
        .map
        .depth_djikstra_map(player_position, Some(MONSTER_FOV_RADIUS));
    // Find all the new positions
    chasers.iter_mut().for_each(|(entity, _, fov, p)| {
        if fov.visible_positions.contains(player_position) {
            let destination = dmap.next_along_path(p);

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
        }
    });
}
