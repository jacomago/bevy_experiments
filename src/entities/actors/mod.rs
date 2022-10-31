use bevy::prelude::*;

mod monsters;
mod npc;
mod player;

use iyes_loopless::prelude::ConditionSet;
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
use crate::stages::end_turn;
use crate::stages::GameStage;
use crate::stages::TurnState;
use crate::systems::chasing_player::chase_player;
use crate::systems::combat::combat;
use crate::systems::fov::fov;
use crate::systems::fov::FieldOfView;
use crate::systems::movement::movement;
use crate::systems::random_actor::random_move;

use self::monsters::MonstersPlugin;
use self::npc::NPCsPlugin;
use self::player::PlayerPlugin;

use super::items::activate;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(MonstersPlugin)
            .add_plugin(NPCsPlugin)
            .add_system_set_to_stage(
                GameStage::GenerateNPCMoves,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::NPCsTurn)
                    .with_system(random_move)
                    .with_system(chase_player)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::NPCActions,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::NPCsTurn)
                    .with_system(activate)
                    .with_system(combat)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MoveNPCs,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::NPCsTurn)
                    .with_system(movement)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::NPCFieldOfView,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::NPCsTurn)
                    .with_system(fov)
                    .with_system(end_turn)
                    .into(),
            );
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
