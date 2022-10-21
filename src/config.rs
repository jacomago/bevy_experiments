use std::env;

use bevy::prelude::Plugin;
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let config = Settings::new().unwrap();
        app.insert_resource(config);
    }
}

#[derive(Debug, Deserialize)]
pub struct ActorSettings {
    pub entity: EntitySettings,
    pub max_health: i32,
    pub fov_radius: i32,
}

#[derive(Debug, Deserialize)]
pub struct EntitySettings {
    pub sprite_index: usize,
    pub name: String,
}

#[derive(Debug, Deserialize, Default)]
pub enum Behaviour {
    Random,
    #[default]
    Chasing,
}

#[derive(Debug, Deserialize)]
pub struct MonsterSettings {
    pub actor: ActorSettings,
    pub behaviour: Behaviour,
    pub proportion: f64,
}

#[derive(Debug, Deserialize)]
pub struct MonstersSettings {
    pub monsters: Vec<MonsterSettings>,
    pub amount: usize,
    pub z_level: f32,
}

#[derive(Debug, Deserialize)]
pub struct MapSettings {
    pub floor_sprite_index: usize,
    pub wall_sprite_index: usize,
    pub width: usize,
    pub height: usize,
    pub z_level: f32,
    pub architect: Architect,
}

#[derive(Debug, Deserialize, Default)]
pub enum Architect {
    Empty,
    #[default]
    Standard,
    Automata,
    Drunkard,
}

#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
pub enum ItemType {
    Healing,
    #[default]
    DungeonMap,
}

#[derive(Debug, Deserialize)]
pub struct ItemSettings {
    pub entity: EntitySettings,
    pub proportion: f64,
    pub item_type: ItemType,
}

#[derive(Debug, Deserialize)]
pub struct ItemsSettings {
    pub items: Vec<ItemSettings>,
    pub winitem: EntitySettings,
    pub z_level: f32,
    pub amount: i32,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub tile_size: i32,
    pub max_fov: i32,
    pub monsters_settings: MonstersSettings,
    pub map_settings: MapSettings,
    pub items_settings: ItemsSettings,
    pub player_settings: ActorSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("cake"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;

    #[test]
    fn test_load() {
        let config = Settings::new();
        dbg!(&config);
        assert!(&config.is_ok())
    }
}
