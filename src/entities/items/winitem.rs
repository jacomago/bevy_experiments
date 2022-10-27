use bevy::prelude::*;

use crate::{
    config::Settings,
    entities::{GameEntityBundle, MapLevel},
    loading::TextureAtlasAssets,
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
    player_level: Query<&MapLevel>,
) {
    let level = player_level.get_single();
    if level.is_err() || settings.end_level - 1 > level.unwrap().value {
        return;
    }
    info!("Spawn winitem");
    commands
        .spawn_bundle(ItemBundle {
            entity: GameEntityBundle::from_settings(
                &settings.items_settings.winitem,
                map_builder.winitem_start,
                &textures.texture_atlas,
                settings.items_settings.z_level,
                settings.tile_size,
            ),
            ..default()
        })
        .insert(WinItem);
}
