use bevy::prelude::*;

use crate::config::ItemType;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Quest {
    pub giver: Option<Entity>,
    pub requested_item: ItemType,
}
