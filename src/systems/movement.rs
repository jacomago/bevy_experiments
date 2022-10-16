use bevy::prelude::*;

use crate::{
    actors::Player,
    camera::focus_camera,
    map::{djikstra::Neighbours, map_builder::MapBuilder, map_position::MapPosition},
};

pub const CHARACTER_Z: f32 = 1.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: MapPosition,
}

pub fn movement(
    mut move_events: EventReader<WantsToMove>,
    mut query: Query<(&mut Transform, &mut MapPosition, Without<Camera2d>)>,
    player_query: Query<Entity, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    map_builder: Res<MapBuilder>,
) {
    let player = player_query.single();
    move_events.iter().for_each(
        |&WantsToMove {
             entity,
             destination,
         }| {
            if map_builder.map.can_enter_tile(&destination) {
                if let Ok((mut transform, mut position, _)) = query.get_mut(entity) {
                    transform.translation = destination.translation(CHARACTER_Z);
                    position.position = destination.position;

                    // If moving player also move camera
                    if entity == player {
                        focus_camera(&mut camera_query, transform);
                    }
                }
            }
        },
    );
}
