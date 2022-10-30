use bevy::prelude::*;

use crate::config::ItemType;

pub struct Quest {
    giver: Entity,
    requested_item: ItemType,
}
