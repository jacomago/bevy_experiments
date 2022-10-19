pub mod grid_map;
pub mod map_builder;
pub mod tile_map;

use bevy::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};

use crate::{config::Settings, GameState};

use self::map_builder::MapBuilder;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Generation).with_system(insert_mapbuilder),
        )
        .add_system_set(SystemSet::on_update(GameState::Generation).with_system(end_gen));
    }
}

fn insert_mapbuilder(mut commands: Commands, mut rng: ResMut<GlobalRng>, settings: Res<Settings>) {
    commands.insert_resource(MapBuilder::new(
        RngComponent::from(&mut rng),
        settings.map_settings.height,
        settings.map_settings.width,
    ));
}

fn end_gen(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Playing).unwrap();
}
