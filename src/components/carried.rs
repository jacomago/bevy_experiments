use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct Carried {
    pub entity: Entity,
}
