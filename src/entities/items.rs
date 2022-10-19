use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components,
    components::{map_position::MapPosition, name::CharacterName},
    config::Settings,
    game_ui::tooltip::Interactive,
    loading::TextureAtlasAssets,
    map::map_builder::MapBuilder,
    GameState,
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_wintitem))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Item>),
            );
    }
}

#[derive(Component, Default)]
pub struct Item;

#[derive(Component)]
pub struct WinItem;

#[derive(Bundle, Default)]
pub struct ItemBundle {
    _i: Item,
    pub name: CharacterName,
    pub position: MapPosition,
    pub interactive: Interactive,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub fn spawn_wintitem(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    settings: Res<Settings>,
) {
    let position = map_builder.winitem_start;
    commands
        .spawn_bundle(ItemBundle {
            name: CharacterName(settings.items_settings.winitem_name.clone()),
            position,
            interactive: Interactive {
                text: settings.items_settings.winitem_name.clone(),
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position
                        .translation(settings.items_settings.z_level, settings.tile_size),
                    ..default()
                },
                texture_atlas: textures.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: settings.items_settings.winitem_sprite_index,
                    ..default()
                },

                ..default()
            },
            ..default()
        })
        .insert(WinItem);
}
