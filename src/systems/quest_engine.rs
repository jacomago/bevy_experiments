use bevy::prelude::*;

use crate::{
    cleanup::cleanup_components,
    entities::{AvailableQuest, Player, QuestState, Reward},
    systems::inventory::Carried,
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

pub fn interact_quest_giver(
    mut commands: Commands,
    mut quest_events: EventReader<InteractQuestGiver>,
    mut assigned_quests: Query<(&mut QuestState, &AssignedQuest)>,
    rewards: Query<&Reward>,
) {
    quest_events.iter().for_each(|event| {
        info!("Interact quest giver");
        if let Ok((mut s, aq)) = assigned_quests.get_mut(event.quest) {
            if *s == QuestState::Updated {
                *s = QuestState::Completed;
                // If reward exists, assign to the Quest assignee
                if let Ok(reward) = rewards.get(event.quest) {
                    commands.entity(reward.0).insert(Carried {
                        entity: aq.assignee,
                    });
                }
            }
        } else {
            commands.entity(event.quest).insert(AssignedQuest {
                assignee: event.reciever,
            });
        }
    });
}

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
    pub updated: Vec<Entity>,
    pub completed: Vec<Entity>,
    pub is_dirty: bool,
}

pub fn update_quests(
    player_query: Query<(Entity, With<Player>)>,
    all_assigned_quests: Query<(Entity, &QuestState, &AssignedQuest)>,
    mut quests_query: Query<&mut PlayerQuests>,
) {
    let mut quests = quests_query.single_mut();
    let (player, _) = player_query.single();
    let player_quests = all_assigned_quests
        .iter()
        .filter(|(_, _, c)| c.assignee == player)
        .map(|(e, s, _)| (e, s));

    let mut assigned = Vec::new();
    let mut completed = Vec::new();
    let mut updated = Vec::new();

    player_quests.for_each(|(e, s)| match s {
        QuestState::Completed => completed.push(e),
        QuestState::Updated => updated.push(e),
        QuestState::Todo => assigned.push(e),
    });
    if quests.assigned != assigned || quests.updated != updated {
        quests.is_dirty = true;
        quests.assigned = assigned;
        quests.completed = completed;
        quests.updated = updated;
    }
}

pub fn update_quest_giver_display(
    mut quest_giver: Query<(&mut TextureAtlasSprite, &AvailableQuest)>,
    changed_quests: Query<&QuestState, Changed<QuestState>>,
) {
    quest_giver.iter_mut().for_each(|(mut sprite, q)| {
        if let Ok(s) = changed_quests.get(q.0) {
            let state_color = match s {
                QuestState::Todo => Color::default(),
                QuestState::Updated => Color::GREEN,
                QuestState::Completed => Color::DARK_GRAY,
            };
            if sprite.color != state_color {
                sprite.color = state_color;
            }
        }
    });
}
