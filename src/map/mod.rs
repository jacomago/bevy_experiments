pub mod grid_map;
pub mod map_builder;
pub mod tile_map;

use bevy::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    config::Settings,
    entities::{MapLevel, TileType},
    stages::TurnState,
    GameState,
};

use self::{grid_map::base_map::BaseMap, map_builder::MapBuilder};

pub struct MapPlugin;

pub const GEN_MAP_LABEL: &str = "GenearateMapBuilder";

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Generation).with_system(insert_mapbuilder),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(respawn_map.run_if_resource_equals(TurnState::NextLevel))
                .label(GEN_MAP_LABEL),
        )
        .add_system_set(SystemSet::on_update(GameState::Generation).with_system(end_gen));
    }
}

fn respawn_map(
    mut map_builder: ResMut<MapBuilder>,
    mut rng: ResMut<GlobalRng>,
    settings: Res<Settings>,
    level: Query<&MapLevel>,
) {
    let mut mb = MapBuilder::new(
        RngComponent::from(&mut rng),
        settings.map_settings.height,
        settings.map_settings.width,
        &settings.map_settings.architect,
    );

    #[cfg(debug_assertions)]
    {
        info!("map {}", mb);
    }

    if settings.end_level - 1 > level.single().value {
        mb.map.set(mb.winitem_start, TileType::Exit);
    }
    *map_builder = mb;
}

fn insert_mapbuilder(mut commands: Commands, mut rng: ResMut<GlobalRng>, settings: Res<Settings>) {
    let mut mb = MapBuilder::new(
        RngComponent::from(&mut rng),
        settings.map_settings.height,
        settings.map_settings.width,
        &settings.map_settings.architect,
    );
    mb.map.set(mb.winitem_start, TileType::Exit);
    commands.insert_resource(mb);
}

fn end_gen(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Playing).unwrap();
}
