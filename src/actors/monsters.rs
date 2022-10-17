//! Monsters:
//!
//! - Nutritionist
//! - Yoga Bunny
//! - Gym Bro
//! - Supplement Pusher

use crate::cleanup::cleanup_components;
use crate::game_ui::tooltip::Interactive;
use crate::map::map_builder::MapBuilder;
use crate::map::map_position::MapPosition;
use crate::stages::{end_turn, TurnState};
use crate::systems::chasing_player::{chase_player, ChasingPlayer};
use crate::systems::combat::combat;
use crate::systems::fov::{fov, FieldOfView};
use crate::systems::movement::{movement, CHARACTER_Z};
use crate::systems::random_actor::{random_move, RandomMover};
use crate::GameState;
use crate::{loading::TextureAtlasAssets, stages::GameStage};

use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use iyes_loopless::prelude::ConditionSet;

use super::components::health::Health;
use super::components::name::CharacterName;

pub struct MonstersPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_monsters))
            .add_system_set_to_stage(
                GameStage::GenerateMonsterMoves,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(random_move)
                    .with_system(chase_player)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MonsterCombat,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(combat)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MoveMonsters,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(movement)
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MonsterFOV,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(fov)
                    .with_system(end_turn)
                    .into(),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(cleanup_components::<Monster>),
            );
    }
}
#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    _m: Monster,
    pub name: CharacterName,
    pub position: MapPosition,
    pub interactive: Interactive,
    pub health: Health,
    pub fov: FieldOfView,
    #[bundle]
    sprite: SpriteSheetBundle,
}

enum Behaviour {
    Random,
    Chasing,
}

struct MonsterConfig {
    name: String,
    health: i32,
    sprite_index: usize,
    behaviour: Behaviour,
}

fn nutritionist() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 111,
        name: "Nutritionist".to_string(),
        health: 2,
        behaviour: Behaviour::Random,
    }
}

fn yoga_bunny() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 69,
        name: "Yoga Bunny".to_string(),
        health: 1,
        behaviour: Behaviour::Chasing,
    }
}

fn gym_bro() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 79,
        name: "Gym Bro".to_string(),
        health: 4,
        behaviour: Behaviour::Chasing,
    }
}

fn supplement_pusher() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 103,
        name: "Supplement Pusher".to_string(),
        health: 3,
        behaviour: Behaviour::Chasing,
    }
}

fn spawn_monsters(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    mut rng: ResMut<GlobalRng>,
) {
    map_builder.rooms.iter().skip(1).for_each(|room| {
        let position = MapPosition::new(room.x() as i32, room.y() as i32);
        let rng_comp = RngComponent::from(&mut rng);
        spawn_monster(&mut commands, position, &textures, rng_comp);
    });
}

fn spawn_monster(
    commands: &mut Commands,
    position: MapPosition,
    textures: &Res<TextureAtlasAssets>,
    mut rng: RngComponent,
) {
    let config = match rng.usize(0..100) {
        0..=60 => yoga_bunny(),
        61..=80 => gym_bro(),
        81..=95 => nutritionist(),
        _ => supplement_pusher(),
    };
    let mut monster = commands.spawn_bundle(MonsterBundle {
        name: CharacterName(config.name.clone()),
        position,
        health: Health {
            current: config.health,
            max: config.health,
        },
        fov: FieldOfView::new(6),
        interactive: Interactive {
            text: format!("{} hp:{}", &config.name, config.health),
        },
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: position.translation(CHARACTER_Z),
                ..default()
            },
            texture_atlas: textures.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: config.sprite_index,
                ..default()
            },

            ..default()
        },
        ..default()
    });
    match &config.behaviour {
        Behaviour::Random => monster.insert(RandomMover { rng }),
        Behaviour::Chasing => monster.insert(ChasingPlayer {}),
    };
}
