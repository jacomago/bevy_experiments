use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::{
    map::map_position::MapPosition,
    monsters::Monster,
    player::Player,
    stages::{GameStage, TurnState, end_turn},
};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            GameStage::Collisions,
            ConditionSet::new()
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(collisions)
                .with_system(end_turn)
                .into(),
        );
    }
}

fn collisions(
    mut commands: Commands,
    monsters: Query<(Entity, &MapPosition, With<Monster>)>,
    player_position_query: Query<&MapPosition, With<Player>>,
) {
    let player_position = player_position_query.single();
    monsters
        .iter()
        .filter(|(_, p, _)| *p == player_position)
        .for_each(|(e, _, _)| commands.entity(e).despawn_recursive());
}
