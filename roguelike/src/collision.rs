use crate::prelude::{map_position::MapPosition, *};

pub fn collisions(
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