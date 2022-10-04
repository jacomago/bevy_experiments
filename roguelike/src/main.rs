mod camera;
mod collision;
mod map;
mod monsters;
mod player;

mod prelude {
    pub const SCREEN_WIDTH: usize = 80;
    pub const SCREEN_HEIGHT: usize = 50;
    pub const TILE_SIZE: i32 = 32;
    pub const MAP_Z: f32 = 0.0;
    pub const PLAYER_Z: f32 = 1.0;
    pub const MONSTER_Z: f32 = 1.0;
    pub use crate::camera::*;
    pub use crate::collision::*;
    pub use crate::map::*;
    pub use crate::monsters::*;
    pub use crate::player::*;
    pub use bevy::prelude::*;
}

use bevy_inspector_egui::WorldInspectorPlugin;
use prelude::{map_builder::MapBuilder, tile_map::TileMap, *};

#[derive(Default)]
pub struct Game {
    map: TileMap,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
    GameOver,
}

fn sprite_sheet_setup(
    asset_server: &Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load("images/dungeonfont.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        16,
        17,
    );

    texture_atlases.add(texture_atlas)
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game: ResMut<Game>,
) {
    let map_builder = MapBuilder::new();

    let texture_atlas_handle = sprite_sheet_setup(&asset_server, texture_atlases);

    player::setup(
        &mut commands,
        &texture_atlas_handle,
        map_builder.player_start,
    );
    monsters::setup(&mut commands, &texture_atlas_handle, &map_builder.rooms);

    game.map = map_builder.map;
    game.map.setup(&mut commands, &texture_atlas_handle);
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
                .with_system(focus_camera)
                .with_system(collisions),
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
