use bevy::{math::ivec2, prelude::*};
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    actors::{Monster, Player},
    map::map_position::MapPosition,
};

use super::{combat::WantsToAttack, movement::WantsToMove};

#[derive(Component, Default)]
pub struct RandomMover {
    pub rng: RngComponent,
}

pub fn random_move(
    player_query: Query<(Entity, &MapPosition, With<Player>)>,
    all_positions: Query<&MapPosition, With<Monster>>,
    mut random_movers: Query<(Entity, &mut RandomMover, &MapPosition)>,
    mut move_events: EventWriter<WantsToMove>,
    mut combat_events: EventWriter<WantsToAttack>,
) {
    let (player, player_position, _) = player_query.single();

    // Find all the new positions
    random_movers.iter_mut().for_each(|(entity, mut rng, p)| {
        let destination = MapPosition::from_ivec2(
            match rng.rng.usize(0..4) {
                0 => ivec2(-1, 0),
                1 => ivec2(1, 0),
                2 => ivec2(0, -1),
                _ => ivec2(0, 1),
            } + p.position,
        );

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
