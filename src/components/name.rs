use std::fmt::Display;

use bevy::prelude::Component;

#[derive(Debug, Clone, PartialEq, Eq, Component, Default)]
pub struct EntityName(pub String);

impl Display for EntityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", &self.0))
    }
}
