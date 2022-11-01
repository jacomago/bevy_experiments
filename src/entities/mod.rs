use bevy::prelude::*;

use crate::components::map_position::MapPosition;
use crate::components::name::EntityName;
use crate::config::EntitySettings;
use crate::game_ui::tooltip::Interactive;

use self::actors::ActorsPlugin;
use self::quest::QuestPlugin;
use self::{items::ItemsPlugin, tile::TilePlugin};

mod actors;
mod items;
mod quest;
mod tile;

pub use actors::AvailableQuest;
pub use actors::MapLevel;
pub use actors::Monster;
pub use actors::Npc;
pub use actors::Player;
pub use items::ActivateItem;
pub use items::Item;
pub use items::ProvidesHealing;
pub use items::ProvidesMap;
pub use items::Weapon;
pub use items::WinItem;
pub use quest::FetchItem;
pub use quest::Quest;
pub use quest::QuestState;
pub use tile::Tile;
pub use tile::TileType;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActorsPlugin)
            .add_plugin(ItemsPlugin)
            .add_plugin(TilePlugin)
            .add_plugin(QuestPlugin);
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
