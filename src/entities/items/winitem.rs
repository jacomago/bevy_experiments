use bevy::prelude::*;

use crate::{
    components::name::CharacterName, config::Settings, game_ui::tooltip::Interactive,
    loading::TextureAtlasAssets, map::map_builder::MapBuilder,
};

use super::ItemBundle;

#[derive(Component)]
pub struct WinItem;

pub fn spawn_wintitem(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    settings: Res<Settings>,
) {
    let position = map_builder.winitem_start;
    commands
        .spawn_bundle(ItemBundle {
            name: CharacterName(settings.items_settings.winitem.name.clone()),
            position,
            interactive: Interactive {
                text: settings.items_settings.winitem.name.clone(),
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position
                        .translation(settings.items_settings.z_level, settings.tile_size),
                    ..default()
                },
                texture_atlas: textures.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: settings.items_settings.winitem.sprite_index,
                    ..default()
                },

                ..default()
            },
            ..default()
        })
        .insert(WinItem);
}
