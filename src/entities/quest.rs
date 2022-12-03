use bevy::prelude::*;

use crate::{
    components::name::EntityName,
    config::{ItemType, QuestSettings},
};

use super::{items::insert_item_type, Item};

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Debug, Component, PartialEq, Eq)]
pub enum QuestState {
    Todo,
    Updated,
    Completed,
}

#[derive(Debug, Component, Default)]
pub struct Quest;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct FetchItem {
    pub requested_item: ItemType,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Reward(pub Entity);

#[derive(Debug, Bundle)]
pub struct QuestBundle {
    _q: Quest,
    name: EntityName,
    fetch_item: FetchItem,
    state: QuestState,
}

pub fn spawn_quest(commands: &mut Commands, quest_setting: &QuestSettings) -> Entity {
    let reward_id = if let Some(reward_config) = &quest_setting.reward {
        let mut reward = commands.spawn(Item);
        reward.insert(EntityName(reward_config.entity.name.clone()));
        insert_item_type(
            &mut reward,
            &reward_config.item_type,
            reward_config.entity.base_damage,
            reward_config.effect_amount,
        );
        Some(reward.id())
    } else {
        None
    };
    let mut quest = commands.spawn(QuestBundle {
        _q: Default::default(),
        name: EntityName(quest_setting.name.clone()),
        fetch_item: FetchItem {
            requested_item: quest_setting.item_type,
        },
        state: QuestState::Todo,
    });
    if let Some(r_id) = reward_id {
        quest.insert(Reward(r_id));
    }
    quest.id()
}
