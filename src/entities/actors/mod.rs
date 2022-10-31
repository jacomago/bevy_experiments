use bevy::prelude::*;

mod monsters;
mod npc;
mod player;

pub use monsters::Monster;
pub use npc::AvailableQuest;
pub use npc::Npc;
pub use player::MapLevel;
pub use player::Player;

use crate::components::health::Health;
use crate::components::map_position::MapPosition;
use crate::components::name::EntityName;
use crate::config::ActorSettings;
use crate::game_ui::tooltip::Interactive;
use crate::systems::fov::FieldOfView;

use self::monsters::MonstersPlugin;
use self::npc::NPCsPlugin;
use self::player::PlayerPlugin;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MonstersPlugin)
            .add_plugin(NPCsPlugin);
    }
}

#[derive(Bundle, Default)]
pub struct ActorBundle {
    pub name: EntityName,
    pub position: MapPosition,
    pub interactive: Interactive,
    pub fov: FieldOfView,
    pub health: Health,
    #[bundle]
    sprite: SpriteSheetBundle,
}

impl ActorBundle {
    fn from_settings(
        settings: &ActorSettings,
        position: MapPosition,
        texture_atlas: &Handle<TextureAtlas>,
        z_level: f32,
        tile_size: i32,
    ) -> Self {
        Self {
            name: EntityName(settings.entity.name.clone()),
            position,
            health: Health {
                current: settings.max_health,
                max: settings.max_health,
            },
            interactive: Interactive {
                text: format!("{} hp:{}", &settings.entity.name, settings.max_health),
            },
            fov: FieldOfView::new(settings.fov_radius),
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position.translation(z_level, tile_size),
                    ..default()
                },
                texture_atlas: texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: settings.entity.sprite_index,
                    ..default()
                },

                ..default()
            },
        }
    }
}
