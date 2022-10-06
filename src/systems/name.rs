use bevy::prelude::Component;

#[derive(Debug, Clone, PartialEq, Eq, Component, Default)]
pub struct CharacterName (pub String);