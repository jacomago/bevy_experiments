use bevy::prelude::*;

use crate::Game;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// change the focus of the camera
pub fn focus_camera(
    game: ResMut<Game>,
    mut transforms: ParamSet<(Query<&mut Transform, With<Camera2d>>, Query<&Transform>)>,
) {
    // if there is both a player and a bonus, target the mid-point of them
    if let Some(player_entity) = game.player.entity {
        let mut transform = None;
        if let Ok(player_transform) = transforms.p1().get(player_entity) {
            transform = Some(player_transform.translation);
        }
        match transform {
            Some(t) => {
                for mut transform in transforms.p0().iter_mut() {
                    *transform = transform.looking_at(t, Vec3::Y);
                }
            }
            None => {}
        }
        // otherwise, target the middle
    }
}
