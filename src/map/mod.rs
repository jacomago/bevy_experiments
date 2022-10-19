pub mod grid_map;
pub mod map_builder;
pub mod tile_map;

use bevy::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};

use crate::GameState;

use self::map_builder::MapBuilder;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Generation).with_system(insert_mapbuilder),
        )
        .add_system_set(SystemSet::on_update(GameState::Generation).with_system(end_gen));
    }
}

fn insert_mapbuilder(mut commands: Commands, mut rng: ResMut<GlobalRng>) {
    commands.insert_resource(MapBuilder::new(
        RngComponent::from(&mut rng),
        MAP_HEIGHT,
        MAP_WIDTH,
    ));
}

fn end_gen(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Playing).unwrap();
}
