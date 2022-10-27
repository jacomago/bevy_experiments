//! This code is probably way more complicated than it needs to be.
//! We have Four different stage systems, GameState, GameStage, TurnState
//! and labels.
//!
//! GameState is the top level and uses the Bevy state system.
//! TurnState is the next level as a resource and uses the iyes_loopless crate.
//! GameStage is the next level and uses the Bevy Stage system.
//! Finally for TurnState::NextLevel we use labels.

//! Generally we go:
//!
//!  - GameState::Menu
//!  - GameState::Generate (generate MapBuilder)
//!  - GameState::Playing
//!      - TurnState::AwaitingInput
//!      - TurnState::PlayerTurn
//!         - GameStage::PlayerCombat (and use items)
//!         - GameStage::MovePlayer
//!         - GameStage::PlayerFOV
//!      - TurnState::MonsterTurn
//!         - GameStage::MonsterCombat
//!         - GameStage::MoveMonsters
//!         - GameStage::MonsterFOV
//!      - TurnState::NextLevel
//!         - systems before GEN_MAP_LABEL
//!         - system GEN_MAP_LABEL
//!         - systems after GEN_MAP_LABEL (mostly labelled RESPAWN_LABEL)
//!         - system after RESPAWN_LABEL (advance_level)
//!     - back to TurnState::AwaitingInput
//!     - possible TurnState::GameOver or TurnState::Victory
//!  - Return to GameState::Menu
//!         

use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    components::{health::Health, map_position::MapPosition},
    entities::{MapLevel, Player, TileType, WinItem, RESPAWN_LABEL},
    map::{grid_map::base_map::BaseMap, map_builder::MapBuilder},
    menu::{PlayerMessage, LOST_MESSAGE, WELCOME_MESSAGE, WIN_MESSAGE},
    GameState,
};

/// Stages of a game, per turn
#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    /// Move the player
    MovePlayer,
    /// Calculate Player Field of view
    PlayerFOV,
    /// Stage where player attacks monsters
    PlayerCombat,
    /// Generate the mve of the monsters
    GenerateMonsterMoves,
    /// Do any required movements
    MoveMonsters,
    /// Calculate Monster field of view
    MonsterFOV,
    /// Let the monsters attack the player
    MonsterCombat,
}

/// States a turn can be in
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum TurnState {
    /// Waiting input from the player
    #[default]
    AwaitingInput,
    /// The players turn
    PlayerTurn,
    /// The Monster#s turn
    MonsterTurn,
    /// The game has been lost
    GameOver,
    /// The game has been won
    Victory,
    /// Moving Level
    NextLevel,
}

/// Plugin for adding all the stages and the sequences of them
pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_stage_after(
            CoreStage::Update,
            GameStage::PlayerCombat,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::PlayerCombat,
            GameStage::MovePlayer,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MovePlayer,
            GameStage::PlayerFOV,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::PlayerFOV,
            GameStage::GenerateMonsterMoves,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::GenerateMonsterMoves,
            GameStage::MonsterCombat,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MonsterCombat,
            GameStage::MoveMonsters,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MoveMonsters,
            GameStage::MonsterFOV,
            SystemStage::parallel(),
        )
        .init_resource::<TurnState>()
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(end_game.run_if_resource_equals(TurnState::GameOver))
                .with_system(end_game.run_if_resource_equals(TurnState::Victory))
                .with_system(
                    advance_level
                        .run_if_resource_equals(TurnState::NextLevel)
                        .after(RESPAWN_LABEL),
                ),
        );
    }
}

/// After each turn we use this to figure out what the next turn is
pub fn end_turn(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    win_item: Query<(&MapPosition, With<WinItem>)>,
    player: Query<(&Health, &MapPosition, With<Player>)>,
    map_builder: Res<MapBuilder>,
) {
    info!("end turn: {:?}", turn_state);
    let (player_health, player_position, _) = player.single();
    let (win_item_position, _) = win_item.single();
    let new_state: TurnState = if player_health.current < 1 {
        TurnState::GameOver
    } else if map_builder.map.value(player_position) == TileType::Exit {
        TurnState::NextLevel
    } else if player_position == win_item_position {
        TurnState::Victory
    } else {
        match *turn_state {
            // In the source project, AwaitingInput returns AwaitingInput, however, it's actually an unreachable
            // case, because the change to the next state (PlayerTurn) is performed in the `player_input` system.
            TurnState::AwaitingInput => unreachable!(),
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            _ => *turn_state,
        }
    };

    commands.insert_resource(new_state);
}

/// Trigures the game over
fn end_game(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    turn_state: Res<TurnState>,
) {
    commands.insert_resource(PlayerMessage {
        message: match *turn_state {
            TurnState::GameOver => LOST_MESSAGE.to_owned(),
            TurnState::Victory => WIN_MESSAGE.to_owned(),
            _ => WELCOME_MESSAGE.to_owned(),
        },
    });
    commands.insert_resource(TurnState::AwaitingInput);

    state.set(GameState::Menu).unwrap();
}

/// Trigures the change of level
pub fn advance_level(
    mut commands: Commands,
    mut player_query: Query<(&mut MapLevel, With<Player>)>,
) {
    let (mut level, _) = player_query.single_mut();
    // increase player level
    level.value += 1;

    commands.insert_resource(TurnState::AwaitingInput);
}
