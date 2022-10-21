use bevy::prelude::*;

use crate::{
    config::Settings, entities::GameEntityBundle, loading::TextureAtlasAssets,
    map::map_builder::MapBuilder,
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
    commands
        .spawn_bundle(ItemBundle {
            entity: GameEntityBundle::from_settings(
                &settings.items_settings.winitem,
                &map_builder.winitem_start,
                &textures.texture_atlas,
                settings.items_settings.z_level,
                settings.tile_size,
            ),
            ..default()
        })
        .insert(WinItem);
}
