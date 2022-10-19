use bevy::prelude::*;

use crate::{
    camera::focus_camera,
    components::map_position::MapPosition,
    config::Settings,
    entities::Player,
    map::{grid_map::base_map::BaseMap, map_builder::MapBuilder},
};

use super::fov::FieldOfView;

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
    mut fovs: Query<&mut FieldOfView>,
    player_query: Query<Entity, With<Player>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    map_builder: Res<MapBuilder>,
    settings: Res<Settings>,
) {
    let player = player_query.single();
    let tile_size = settings.tile_size;
    move_events.iter().for_each(
        |&WantsToMove {
             entity,
             destination,
         }| {
            if map_builder.map.can_enter_tile(&destination) {
                if let Ok((mut transform, mut position, _)) = query.get_mut(entity) {
                    transform.translation = destination.translation(CHARACTER_Z, tile_size);
                    position.position = destination.position;

                    // If moving player also move camera
                    if entity == player {
                        focus_camera(&mut camera_query, transform);
                    }
                    if let Ok(mut fov) = fovs.get_mut(entity) {
                        *fov = fov.clone_dirty();
                    }
                }
            }
        },
    );
}
