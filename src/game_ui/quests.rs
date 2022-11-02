use bevy::prelude::*;

use crate::{
    components::name::EntityName,
    systems::quest_engine::{AssignedQuest, PlayerQuests},
};

use super::hud::UiState;

pub fn update_quests_hud(
    mut quests_query: Query<&mut PlayerQuests>,
    assigned_quests: Query<(With<AssignedQuest>, &EntityName)>,
    mut ui_status: ResMut<UiState>,
) {
    let mut quests = quests_query.single_mut();
    if !quests.is_dirty {
        return;
    }

    ui_status.quests.assigned = quests
        .assigned
        .iter()
        .map(|entity| {
            let (_, name) = assigned_quests.get(*entity).unwrap();
            format!("{}", name)
        })
        .collect();
    ui_status.quests.updated = quests
        .updated
        .iter()
        .map(|entity| {
            let (_, name) = assigned_quests.get(*entity).unwrap();
            format!("{}", name)
        })
        .collect();
    ui_status.quests.completed = quests
        .completed
        .iter()
        .map(|entity| {
            let (_, name) = assigned_quests.get(*entity).unwrap();
            format!("{}", name)
        })
        .collect();
    quests.is_dirty = false;
}
