use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    MovePlayer,
    MoveMonsters,
    MonsterCollisions,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum TurnState {
    #[default]
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_stage_after(
            CoreStage::Update,
            GameStage::MovePlayer,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MovePlayer,
            GameStage::MoveMonsters,
            SystemStage::parallel(),
        )
        .add_stage_after(
            GameStage::MoveMonsters,
            GameStage::MonsterCollisions,
            SystemStage::parallel(),
        )
        .init_resource::<TurnState>();
    }
}

pub fn end_turn(mut commands: Commands, turn_state: Res<TurnState>) {
    info!("end turn: {:?}", turn_state);
    let new_state = match *turn_state {
        // In the source project, AwaitingInput returns AwaitingInput, however, it's actually an unreachable
        // case, because the change to the next state (PlayerTurn) is performed in the `player_input` system.
        TurnState::AwaitingInput => unreachable!(),
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };

    commands.insert_resource(new_state);
}
