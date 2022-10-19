use std::fmt::Display;

use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components, components::map_position::MapPosition,
    loading::TextureAtlasAssets, map::map_builder::MapBuilder, GameState,
};

const MAP_Z: f32 = 0.0;
const WALL_SPRITE_INDEX: usize = 35;
const FLOOR_SPRITE_INDEX: usize = 46;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_map))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Tile>),
            );
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug, Component)]
pub enum TileType {
    Wall,
    #[default]
    Floor,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Wall => f.write_fmt(format_args!("#")),
            TileType::Floor => f.write_fmt(format_args!(".")),
        }
    }
}

pub fn spawn_map(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
) {
    map_builder
        .map
        .tiles
        .indexed_iter()
        .for_each(|((y, x), t)| {
            let position = MapPosition::new(x.try_into().unwrap(), y.try_into().unwrap());
            commands.spawn_bundle(TileBundle::new(position, textures.as_ref(), *t));
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
    pub fn new(position: MapPosition, textures: &TextureAtlasAssets, tile_type: TileType) -> Self {
        Self {
            position,
            tile_type,
            sprite: SpriteSheetBundle {
                visibility: Visibility { is_visible: false },
                transform: Transform {
                    translation: position.translation(MAP_Z),
                    ..default()
                },
                texture_atlas: textures.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: match tile_type {
                        TileType::Floor => FLOOR_SPRITE_INDEX,
                        TileType::Wall => WALL_SPRITE_INDEX,
                    },
                    ..default()
                },
                ..default()
            },
            ..default()
        }
    }
}
