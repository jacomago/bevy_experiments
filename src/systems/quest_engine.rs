use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components,
    entities::{AvailableQuest, Player},
    GameState,
};

pub struct InteractQuestGiver {
    pub quest: Entity,
    pub reciever: Entity,
}

#[derive(Debug, Component)]
pub struct AssignedQuest {
    pub assignee: Entity,
}

#[derive(Debug, Component)]
pub struct UpdatedQuest;

#[derive(Debug, Component)]
pub struct CompletedQuest;

pub struct QuestEnginePlugin;

impl Plugin for QuestEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractQuestGiver>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_quests))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_quests)
                    .with_system(update_quest_giver_display),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing)
                    .with_system(cleanup_components::<PlayerQuests>),
            );
    }
}

pub fn assign_quest(
    mut commands: Commands,
    mut quest_events: EventReader<InteractQuestGiver>,
    assigned_quests: Query<Entity, With<AssignedQuest>>,
    updated_quests: Query<Entity, With<UpdatedQuest>>,
) {
    quest_events.iter().for_each(|event| {
        info!("Interact quest giver");
        if assigned_quests.contains(event.quest) {
            if updated_quests.contains(event.quest) {
                commands.entity(event.quest).remove::<UpdatedQuest>();
                commands.entity(event.quest).insert(CompletedQuest);
            }
        } else {
            commands.entity(event.quest).insert(AssignedQuest {
                assignee: event.reciever,
            });
        }
    });
}

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
    pub assigned: Vec<Entity>,
    pub completed: Vec<Entity>,
    pub is_dirty: bool,
}

pub fn update_quests(
    player_query: Query<(Entity, With<Player>)>,
    all_assigned_quests: Query<(Entity, &AssignedQuest)>,
    all_completed_quests: Query<(Entity, &UpdatedQuest)>,
    mut quests_query: Query<&mut PlayerQuests>,
) {
    let mut quests = quests_query.single_mut();
    let (player, _) = player_query.single();
    let player_quests = all_assigned_quests
        .iter()
        .filter(|(_, c)| c.assignee == player)
        .map(|(e, _)| e);

    let mut assigned = Vec::new();
    let mut completed = Vec::new();

    player_quests.for_each(|e| {
        if all_completed_quests.contains(e) {
            completed.push(e);
        } else {
            assigned.push(e);
        }
    });
    if quests.assigned != assigned {
        quests.is_dirty = true;
        quests.assigned = assigned;
        quests.completed = completed;
    }
}

pub fn update_quest_giver_display(
    mut quest_giver: Query<(&mut TextureAtlasSprite, &AvailableQuest)>,
    updated_quests: Query<Entity, Changed<UpdatedQuest>>,
) {
    quest_giver.iter_mut().for_each(|(mut sprite, q)| {
        if updated_quests.contains(q.0) && sprite.color != Color::SEA_GREEN {
            sprite.color = Color::SEA_GREEN;
        }
    });
}
