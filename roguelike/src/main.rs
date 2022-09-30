#![warn(clippy::all, clippy::pedantic)]

mod camera;
mod map;
mod map_builder;
mod player;

mod prelude {
    pub const SCREEN_WIDTH: usize = 80;
    pub const SCREEN_HEIGHT: usize = 50;
    pub const DISPLAY_WIDTH: usize = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: usize = SCREEN_HEIGHT / 2;
    pub const TILE_SIZE: i32 = 32;
    pub use crate::camera::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use bevy::prelude::*;
}
use bevy_inspector_egui::WorldInspectorPlugin;
use prelude::*;
use rand::thread_rng;

#[derive(Default)]
pub struct Game {
    map: Map,
    player: Player,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    let mut rng = thread_rng();
    let map_builder = MapBuilder::new(&mut rng);
    game.map = map_builder.map;
    game.player.position = map_builder.player_start;

    let texture_handle = asset_server.load("images/dungeonfont.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        16,
        17,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    game.player.entity = Some(
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(
                        (game.player.position.x * TILE_SIZE) as f32,
                        (game.player.position.y * TILE_SIZE) as f32,
                        0.0,
                    ),
                    ..default()
                },
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..default()
                },
                ..default()
            })
            .id(),
    );

    game.map.setup(commands, &texture_atlas_handle);
}

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_state(GameState::Playing)
        .add_startup_system(setup_camera)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player)
                .with_system(focus_camera),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
        .add_system(bevy::window::close_on_esc)
        .run();
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
