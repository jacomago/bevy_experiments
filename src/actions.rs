use crate::GameState;
use bevy::{prelude::*, render::camera::RenderTarget};

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(set_movement_actions)
                .with_system(cursor_system),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub mouse_rollover: Option<MousePosition>,
}

#[derive(Default, Debug)]
pub struct MousePosition {
    pub game_position: Vec2,
    pub screen_position: Vec2,
}

fn cursor_system(
    mut actions: ResMut<Actions>,
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width(), wnd.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        dbg!(world_pos);
        actions.mouse_rollover = Some(MousePosition {
            game_position: world_pos,
            screen_position: screen_pos,
        });
    } else {
        actions.mouse_rollover = None;
    }
}

fn set_movement_actions(
    mut actions: ResMut<Actions>,
    mut mut_keyboard_input: ResMut<Input<KeyCode>>,
) {
    let keyboard_input = mut_keyboard_input.as_ref();
    if GameControl::Up.just_released(keyboard_input)
        || GameControl::Up.just_pressed(keyboard_input)
        || GameControl::Left.just_released(keyboard_input)
        || GameControl::Left.just_pressed(keyboard_input)
        || GameControl::Down.just_released(keyboard_input)
        || GameControl::Down.just_pressed(keyboard_input)
        || GameControl::Right.just_released(keyboard_input)
        || GameControl::Right.just_pressed(keyboard_input)
    {
        let mut player_movement = Vec2::ZERO;

        if GameControl::Up.just_released(keyboard_input)
            || GameControl::Down.just_released(keyboard_input)
        {
            if GameControl::Up.pressed(keyboard_input) {
                player_movement.y = 1.;
            } else if GameControl::Down.pressed(keyboard_input) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if GameControl::Up.just_pressed(keyboard_input) {
            player_movement.y = 1.;
        } else if GameControl::Down.just_pressed(keyboard_input) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::ZERO).y;
        }

        if GameControl::Right.just_released(keyboard_input)
            || GameControl::Left.just_released(keyboard_input)
        {
            if GameControl::Right.pressed(keyboard_input) {
                player_movement.x = 1.;
            } else if GameControl::Left.pressed(keyboard_input) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if GameControl::Right.just_pressed(keyboard_input) {
            player_movement.x = 1.;
        } else if GameControl::Left.just_pressed(keyboard_input) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::ZERO).x;
        }

        if player_movement != Vec2::ZERO {
            player_movement = player_movement.normalize();
            info!("Keyboard input made player movement: {}", player_movement);
            actions.player_movement = Some(player_movement);
            mut_keyboard_input.clear();
        }
    } else {
        actions.player_movement = None;
    }
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
}

impl GameControl {
    fn just_released(&self, keyboard_input: &Input<KeyCode>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_released(KeyCode::W)
                    || keyboard_input.just_released(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_released(KeyCode::S)
                    || keyboard_input.just_released(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_released(KeyCode::A)
                    || keyboard_input.just_released(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_released(KeyCode::D)
                    || keyboard_input.just_released(KeyCode::Right)
            }
        }
    }

    fn pressed(&self, keyboard_input: &Input<KeyCode>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
            }
        }
    }

    fn just_pressed(&self, keyboard_input: &Input<KeyCode>) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.just_pressed(KeyCode::W) || keyboard_input.just_pressed(KeyCode::Up)
            }
            GameControl::Down => {
                keyboard_input.just_pressed(KeyCode::S)
                    || keyboard_input.just_pressed(KeyCode::Down)
            }
            GameControl::Left => {
                keyboard_input.just_pressed(KeyCode::A)
                    || keyboard_input.just_pressed(KeyCode::Left)
            }
            GameControl::Right => {
                keyboard_input.just_pressed(KeyCode::D)
                    || keyboard_input.just_pressed(KeyCode::Right)
            }
        }
    }
}
