use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    actors::{components::health::Health, Player, WinItem},
    map::map_position::MapPosition,
    menu::{PlayerMessage, LOST_MESSAGE, WELCOME_MESSAGE, WIN_MESSAGE},
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
                .with_system(end_game.run_if_resource_equals(TurnState::GameOver))
                .with_system(end_game.run_if_resource_equals(TurnState::Victory)),
        );
    }
}

/// After each turn we use this to figure out what the next turn is
pub fn end_turn(
    mut commands: Commands,
    turn_state: Res<TurnState>,
    win_item: Query<(&MapPosition, With<WinItem>)>,
    player: Query<(&Health, &MapPosition, With<Player>)>,
) {
    info!("end turn: {:?}", turn_state);
    let (player_health, player_position, _) = player.single();
    let (win_item_position, _) = win_item.single();
    let new_state: TurnState = if player_health.current < 1 {
        TurnState::GameOver
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
