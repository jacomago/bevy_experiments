//! Monsters:
//!
//! - Nutritionist
//! - Yoga Bunny
//! - Gym Bro
//! - Supplement Pusher

use crate::map::map_builder::MapBuilder;
use crate::map::map_position::MapPosition;
use crate::stages::{end_turn, TurnState};
use crate::systems::health::Health;
use crate::systems::movement::WantsToMove;
use crate::systems::movement::{movement, CHARACTER_Z};
use crate::systems::name::CharacterName;
use crate::GameState;
use crate::{loading::TextureAtlasAssets, stages::GameStage};

use bevy::{math::ivec2, prelude::*};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use iyes_loopless::prelude::ConditionSet;

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
                    .into(),
            )
            .add_system_set_to_stage(
                GameStage::MoveMonsters,
                ConditionSet::new()
                    .run_if_resource_equals(TurnState::MonsterTurn)
                    .with_system(movement)
                    .with_system(end_turn)
                    .into(),
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
    pub health: Health,
    pub rng: RngComponent,
    #[bundle]
    sprite: SpriteSheetBundle,
}

struct MonsterConfig {
    name: String,
    health: i32,
    sprite_index: usize,
}

fn nutritionist() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 111,
        name: "Nutritionist".to_string(),
        health: 2,
    }
}

fn yoga_bunny() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 69,
        name: "Yoga Bunny".to_string(),
        health: 1,
    }
}

fn gym_bro() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 79,
        name: "Gym Bro".to_string(),
        health: 4,
    }
}

fn supplement_pusher() -> MonsterConfig {
    MonsterConfig {
        sprite_index: 103,
        name: "Supplement Pusher".to_string(),
        health: 3,
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
    commands.spawn_bundle(MonsterBundle {
        name: CharacterName(config.name),
        position,
        health: Health {
            current: config.health,
            max: config.health,
        },
        rng,
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
}

fn random_move(
    mut monsters: Query<(Entity, &mut RngComponent, &MapPosition, With<Monster>)>,
    mut move_events: EventWriter<WantsToMove>,
) {
    monsters.iter_mut().for_each(|(entity, mut rng, p, _)| {
        let destination = MapPosition::from_ivec2(
            match rng.usize(0..4) {
                0 => ivec2(-1, 0),
                1 => ivec2(1, 0),
                2 => ivec2(0, -1),
                _ => ivec2(0, 1),
            } + p.position,
        );
        move_events.send(WantsToMove {
            entity,
            destination,
        });
    });
}
