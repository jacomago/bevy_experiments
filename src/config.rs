use std::env;

use bevy::prelude::Plugin;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let config = Settings::new().unwrap();
        app.insert_resource(config);
    }
}

#[derive(Debug, Deserialize)]
pub struct ActorSettings {
    pub name: String,
    pub sprite_index: usize,
    pub max_health: i32,
}

#[derive(Debug, Deserialize)]
pub struct MonstersSettings {
    pub fov_radius: i32,
    pub monsters: Vec<ActorSettings>,
    pub amount: usize,
    pub z_level: usize,
}

#[derive(Debug, Deserialize)]
pub struct MapSettings {
    floor_sprite_index: usize,
    wall_sprite_index: usize,
    width: usize,
    height: usize,
    z_level: usize,
}

#[derive(Debug, Deserialize)]
pub struct ItemsSettings {
    winitem_sprite_index: usize,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub seed: u64,
    pub tile_size: i32,
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
            .add_source(
                File::with_name(&format!("examples/hierarchical-env/config/{}", run_mode))
                    .required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("examples/hierarchical-env/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("cake"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
