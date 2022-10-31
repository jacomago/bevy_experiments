use bevy::prelude::*;

use crate::{cleanup::cleanup_components, entities::Player, GameState};

pub struct RecieveQuest {
    pub quest: Entity,
    pub reciever: Entity,
}

#[derive(Debug, Component)]
pub struct AssignedQuest {
    pub assignee: Entity,
}

#[derive(Debug, Component)]
pub struct CompletedQuest;

pub struct QuestEnginePlugin;

impl Plugin for QuestEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecieveQuest>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_quests))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(update_quests))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(cleanup_components::<PlayerQuests>),
            );
    }
}

pub fn assign_quest(mut commands: Commands, mut quest_events: EventReader<RecieveQuest>) {
    quest_events.iter().for_each(|event| {
        info!("Recieve quest");
        commands.entity(event.quest).insert(AssignedQuest {
            assignee: event.reciever,
        });
    });
}

// TODO mark a quest as completed
// TODO mark quest returned if return to quest giver with compeleted quest
// TODO give reward for returned completed quest

/// Entity for caching the Players Quests
#[derive(Component, Default, Debug)]
pub struct Quests;

#[derive(Bundle, Debug, Default)]
pub struct QuestsBundle {
    _qs: Quests,
    player: PlayerQuests,
}

fn spawn_quests(mut commands: Commands) {
    commands.spawn_bundle(QuestsBundle { ..default() });
}

#[derive(Component, Default, Debug)]
pub struct PlayerQuests {
    pub key_map: Vec<Entity>,
    pub is_dirty: bool,
}

pub fn update_quests(
    player_query: Query<(Entity, With<Player>)>,
    all_assigned_quests: Query<(Entity, &AssignedQuest)>,
    mut quests_query: Query<&mut PlayerQuests>,
) {
    let mut quests = quests_query.single_mut();
    let (player, _) = player_query.single();
    let mut player_quests = all_assigned_quests
        .iter()
        .filter(|(_, c)| c.assignee == player)
        .map(|(e, _)| e)
        .collect::<Vec<_>>();
    player_quests.sort();
    if quests.key_map != player_quests {
        quests.is_dirty = true;
        quests.key_map = player_quests;
    }
}
