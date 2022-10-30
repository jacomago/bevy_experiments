use bevy::prelude::*;

use crate::{config::ItemType, entities::Player};

use super::inventory::Carried;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Quest {
    pub giver: Option<Entity>,
    pub requested_item: ItemType,
}

pub struct RecieveQuest {}

pub struct QuestPlugin;

impl Plugin for QuestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecieveQuest>();
    }
}

pub fn quest(
    mut commands: Commands,
    mut quest_events: EventReader<RecieveQuest>,
    player_query: Query<(Entity, With<Player>)>,
) {
    let (player, _) = player_query.single();
    quest_events.iter().for_each(|event| {
        info!("Recieve quest");
    });
}
