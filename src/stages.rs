use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    actors::{components::health::Health, Player},
    menu::{PlayerMessage, LOST_MESSAGE},
    GameState,
};

/// Stages of a game, per turn
#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    /// Move the player
    MovePlayer,
    /// Stage where player attacks monsters
    PlayerCombat,
    /// Generate the mve of the monsters
    GenerateMonsterMoves,
    /// Do any required movements
    MoveMonsters,
    /// Let the monsters attack the player
    MonsterCombat,
}

/// States a turn can be in
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
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
        .init_resource::<TurnState>()
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(game_over.run_if_resource_equals(TurnState::GameOver)),
        );
    }
}

/// After each turn we use this to figure out what the next turn is
pub fn end_turn(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    player: Query<(&Health, With<Player>)>,
) {
    info!("end turn: {:?}", turn_state);
    let new_state = if player.single().0.current < 1 {
        TurnState::GameOver
    } else {
        match *turn_state {
            // In the source project, AwaitingInput returns AwaitingInput, however, it's actually an unreachable
            // case, because the change to the next state (PlayerTurn) is performed in the `player_input` system.
            TurnState::AwaitingInput => unreachable!(),
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            TurnState::GameOver => TurnState::GameOver,
        }
    };

    commands.insert_resource(new_state);
}

/// Trigures the game over
fn game_over(mut commands: Commands, mut state: ResMut<State<GameState>>) {
    commands.insert_resource(PlayerMessage {
        message: LOST_MESSAGE.to_owned(),
    });
    commands.insert_resource(TurnState::AwaitingInput);

    state.set(GameState::Menu).unwrap();
}
