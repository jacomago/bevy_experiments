use bevy::prelude::*;

use crate::prelude::Player;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// change the focus of the camera to the player
pub fn focus_camera(
    mut transforms: ParamSet<(
        Query<&mut Transform, With<Camera2d>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    let binding = transforms.p1();
    let translation = binding.single().translation;
    let mut binding = transforms.p0();
    let mut camera_transform = binding.single_mut();
    *camera_transform = Transform::from_translation(translation);
}
