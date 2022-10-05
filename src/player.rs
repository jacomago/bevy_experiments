use crate::actions::Actions;
use crate::loading::TextureAtlasAssets;
use crate::map::map_builder::MapBuilder;
use crate::map::map_position::MapPosition;
use crate::stages::{GameStage, TurnState};
use crate::systems::movement::{movement, WantsToMove, CHARACTER_Z};
use crate::GameState;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

const PLAYER_SPRITE_INDEX: usize = 64;

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    _player: Player,
    pub position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(player_input.run_if_resource_equals(TurnState::AwaitingInput)),
            )
            .add_system_set_to_stage(
                GameStage::MovePlayer,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::PlayerTurn)
                    .with_system(movement)
                    .into(),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
) {
    let player_start = map_builder.player_start;
    commands.spawn_bundle(PlayerBundle {
        position: player_start,
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: player_start.translation(CHARACTER_Z),
                ..default()
            },
            texture_atlas: textures.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: PLAYER_SPRITE_INDEX,
                ..default()
            },
            ..default()
        },
        ..default()
    });
}

fn player_input(
    mut commands: Commands,
    actions: Res<Actions>,
    mut move_events: EventWriter<WantsToMove>,
    player_query: Query<(Entity, &MapPosition, With<Player>)>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let movement = actions.player_movement.unwrap().as_ivec2();
    let (entity, position, _) = player_query.single();
    let new_position = MapPosition::from_ivec2(position.position + movement);

    move_events.send(WantsToMove {
        entity,
        destination: new_position,
    });
    commands.insert_resource(TurnState::PlayerTurn);
}
