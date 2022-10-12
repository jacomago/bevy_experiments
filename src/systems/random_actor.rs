use bevy::{math::ivec2, prelude::*};
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{map::map_position::MapPosition, player::Player};

use super::{combat::WantsToAttack, movement::WantsToMove};

#[derive(Component, Default)]
pub struct RandomMover {
    pub rng: RngComponent,
}

pub fn random_move(
    player_query: Query<(Entity, With<Player>)>,
    mut actors: Query<(Entity, &mut RandomMover, &MapPosition)>,
    mut move_events: EventWriter<WantsToMove>,
    mut combat_events: EventWriter<WantsToAttack>,
) {
    // Find all the new positions
    let new_positions: Vec<(Entity, MapPosition)> = actors
        .iter_mut()
        .map(|(entity, mut rng, p)| {
            let destination = MapPosition::from_ivec2(
                match rng.rng.usize(0..4) {
                    0 => ivec2(-1, 0),
                    1 => ivec2(1, 0),
                    2 => ivec2(0, -1),
                    _ => ivec2(0, 1),
                } + p.position,
            );
            (entity, destination)
        })
        .collect();

    let (player, _) = player_query.single();

    // For each new position, check if entity already in it
    new_positions.iter().for_each(|(entity, position)| {
        let mut attack = false;

        actors
            .iter()
            .filter(|(_, _, entity_position)| position == *entity_position)
            .for_each(|(oe, _, _)| {
                // If entity in space is player attack him, else do nothing
                if oe == player {
                    combat_events.send(WantsToAttack {
                        attacker: *entity,
                        victim: oe,
                    });
                }
                attack = true;
            });

        // Move if not attacking anything
        if !attack {
            move_events.send(WantsToMove {
                entity: *entity,
                destination: *position,
            });
        }
    });
}
