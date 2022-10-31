use bevy::prelude::*;

pub struct RecieveQuest {
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
        app.add_event::<RecieveQuest>();
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
