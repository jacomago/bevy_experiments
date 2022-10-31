use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components, config::Settings, map::map_builder::MapBuilder, GameState,
};

/// Plugin to setup the camera
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_camera))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Camera>),
            );
    }
}

/// Insert the game camera
fn setup_camera(mut commands: Commands, map_builder: Res<MapBuilder>, settings: Res<Settings>) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_translation(
            map_builder
                .player_start
                .translation(settings.entity_z_level, settings.tile_size),
        ),
        ..default()
    });
}

/// Focus the camera on the player sprite
pub fn focus_camera(
    camera_query: &mut Query<&mut Transform, With<Camera2d>>,
    transform: Mut<Transform>,
) {
    let mut camera_transform = camera_query.single_mut();
    *camera_transform = Transform::from_translation(transform.translation);
}
