use bevy::prelude::*;

use crate::{map::map_builder::MapBuilder, systems::movement::CHARACTER_Z, GameState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_camera));
    }
}

fn setup_camera(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    map_builder: Res<MapBuilder>,
) {
    let mut camera_transform = camera_query.single_mut();
    *camera_transform =
        Transform::from_translation(map_builder.player_start.translation(CHARACTER_Z));
}

pub fn focus_camera(
    camera_query: &mut Query<&mut Transform, With<Camera2d>>,
    transform: Mut<Transform>,
) {
    let mut camera_transform = camera_query.single_mut();
    *camera_transform = Transform::from_translation(transform.translation);
}
