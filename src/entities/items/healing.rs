use bevy::prelude::Component;

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}
