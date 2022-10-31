use bevy::prelude::*;

use crate::components::map_position::MapPosition;
use crate::components::name::EntityName;
use crate::config::EntitySettings;
use crate::game_ui::tooltip::Interactive;

use self::quest::QuestPlugin;
use self::{
    items::ItemsPlugin, monsters::MonstersPlugin, npc::NPCsPlugin, player::PlayerPlugin,
    tile::TilePlugin,
};

mod items;
mod monsters;
mod npc;
mod player;
mod quest;
mod tile;

pub use items::ActivateItem;
pub use items::Item;
pub use items::Weapon;
pub use items::WinItem;
pub use monsters::Monster;
pub use npc::AvailableQuest;
pub use npc::Npc;
pub use player::MapLevel;
pub use player::Player;
pub use quest::Quest;
pub use tile::Tile;
pub use tile::TileType;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MonstersPlugin)
            .add_plugin(ItemsPlugin)
            .add_plugin(TilePlugin)
            .add_plugin(QuestPlugin)
            .add_plugin(NPCsPlugin);
    }
}

#[derive(Bundle, Default)]
pub struct GameEntityBundle {
    pub name: EntityName,
    pub position: MapPosition,
    pub interactive: Interactive,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub const RESPAWN_LABEL: &str = "RespawnEntities";

impl GameEntityBundle {
    fn from_settings(
        settings: &EntitySettings,
        position: MapPosition,
        texture_atlas: &Handle<TextureAtlas>,
        z_level: f32,
        tile_size: i32,
    ) -> Self {
        Self {
            name: EntityName(settings.name.clone()),
            position,
            interactive: Interactive {
                text: settings.name.clone(),
            },
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position.translation(z_level, tile_size),
                    ..default()
                },
                texture_atlas: texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: settings.sprite_index,
                    ..default()
                },

                ..default()
            },
        }
    }
}
