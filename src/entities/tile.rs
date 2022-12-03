use std::fmt::Display;

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use serde::Deserialize;

use crate::{
    cleanup::cleanup_components,
    components::map_position::MapPosition,
    config::Settings,
    loading::TextureAtlasAssets,
    map::{map_builder::MapBuilder, GEN_MAP_LABEL},
    stages::TurnState,
    GameState,
};

use super::RESPAWN_LABEL;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    spawn_map
                        .run_if_resource_equals(TurnState::NextLevel)
                        .label(RESPAWN_LABEL)
                        .after(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(
                    cleanup_components::<Tile>
                        .run_if_resource_equals(TurnState::NextLevel)
                        .before(GEN_MAP_LABEL),
                ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Tile>),
            );
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, Hash, Component, Deserialize)]
pub enum TileType {
    Wall,
    #[default]
    Floor,
    Exit,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Wall => f.write_fmt(format_args!("#")),
            TileType::Floor => f.write_fmt(format_args!(".")),
            TileType::Exit => f.write_fmt(format_args!(">")),
        }
    }
}

pub fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    settings: Res<Settings>,
) {
    map_builder
        .map
        .tiles
        .indexed_iter()
        .for_each(|((y, x), t)| {
            let position = MapPosition::new(x.try_into().unwrap(), y.try_into().unwrap());
            commands.spawn(TileBundle::new(
                position,
                textures.as_ref(),
                *t,
                settings.tile_size,
                settings.map_settings.z_level,
                settings.map_settings.tile_sprites[t],
            ));
        });
}

#[derive(Component, Default)]
pub struct Tile;

#[derive(Bundle, Default)]
pub struct TileBundle {
    _t: Tile,
    tile_type: TileType,
    position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl TileBundle {
    pub fn new(
        position: MapPosition,
        textures: &TextureAtlasAssets,
        tile_type: TileType,
        tile_size: i32,
        z_level: f32,
        index: usize,
    ) -> Self {
        Self {
            position,
            tile_type,
            sprite: SpriteSheetBundle {
                visibility: Visibility { is_visible: false },
                transform: Transform {
                    translation: position.translation(z_level, tile_size),
                    ..default()
                },
                texture_atlas: textures.texture_atlas.clone(),
                sprite: TextureAtlasSprite { index, ..default() },
                ..default()
            },
            ..default()
        }
    }
}
