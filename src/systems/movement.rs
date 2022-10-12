use bevy::{math::ivec2, prelude::*};
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    camera::focus_camera,
    map::{map_builder::MapBuilder, map_position::MapPosition},
    player::Player,
};

pub const CHARACTER_Z: f32 = 1.;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>();
    }
}

#[derive(Component, Default)]
pub struct RandomMover {
    pub rng: RngComponent,
}

pub fn random_move(
    mut monsters: Query<(Entity, &mut RandomMover, &MapPosition)>,
    mut move_events: EventWriter<WantsToMove>,
) {
    monsters.iter_mut().for_each(|(entity, mut rng, p)| {
        let destination = MapPosition::from_ivec2(
            match rng.rng.usize(0..4) {
                0 => ivec2(-1, 0),
                1 => ivec2(1, 0),
                2 => ivec2(0, -1),
                _ => ivec2(0, 1),
            } + p.position,
        );
        move_events.send(WantsToMove {
            entity,
            destination,
        });
    });
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
    move_events.iter().for_each(
        |&WantsToMove {
             entity,
             destination,
         }| {
            if map_builder.map.can_enter_tile(destination) {
                let (mut transform, mut position, _) = query.get_mut(entity).unwrap();
                transform.translation = destination.translation(CHARACTER_Z);
                position.position = destination.position;

                // If moving player also move camera
                if entity == player_query.single() {
                    focus_camera(&mut camera_query, transform);
                }
            }
        },
    );
}
