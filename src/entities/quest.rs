use bevy::prelude::*;

use crate::{
    components::name::EntityName,
    config::{ItemType, QuestSettings},
};

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Component, Default)]
pub struct Quest;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct FetchItem {
    pub requested_item: ItemType,
}

#[derive(Debug, Bundle)]
pub struct QuestBundle {
    _q: Quest,
    name: EntityName,
    fetch_item: FetchItem,
}

pub fn spawn_quest(commands: &mut Commands, quest_setting: &QuestSettings) -> Entity {
    commands
        .spawn_bundle(QuestBundle {
            _q: Default::default(),
            name: EntityName(quest_setting.name.clone()),
            fetch_item: FetchItem {
                requested_item: quest_setting.item_type,
            },
        })
        .id()
}
