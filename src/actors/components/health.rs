use bevy::prelude::*;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}
