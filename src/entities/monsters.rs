use crate::cleanup::cleanup_components;
use crate::components::health::Health;
use crate::components::map_position::MapPosition;
use crate::components::name::CharacterName;
use crate::config::{Behaviour, MonsterSettings, MonstersSettings, Settings};
use crate::game_ui::tooltip::Interactive;
use crate::map::map_builder::MapBuilder;
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

fn spawn_monsters(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
    mut rng: ResMut<GlobalRng>,
    settings: Res<Settings>,
) {
    let monster_settings = &settings.monsters_settings;
    map_builder.monster_spawns.iter().for_each(|position| {
        let rng_comp = RngComponent::from(&mut rng);
        spawn_monster(
            &mut commands,
            position,
            &textures,
            rng_comp,
            monster_settings,
            settings.tile_size,
        );
    });
}

fn weights(setting: &MonsterSettings) -> f64 {
    0.01 * setting.proportion
}

fn spawn_monster(
    commands: &mut Commands,
    position: &MapPosition,
    textures: &Res<TextureAtlasAssets>,
    mut rng: RngComponent,
    settings: &MonstersSettings,
    tile_size: i32,
) {
    let config = rng.weighted_sample(&settings.monsters, weights).unwrap();
    let mut monster = commands.spawn_bundle(MonsterBundle {
        name: CharacterName(config.actor_settings.name.clone()),
        position: *position,
        health: Health {
            current: config.actor_settings.max_health,
            max: config.actor_settings.max_health,
        },
        fov: FieldOfView::new(config.actor_settings.fov_radius),
        interactive: Interactive {
            text: format!(
                "{} hp:{}",
                &config.actor_settings.name, config.actor_settings.max_health
            ),
        },
        sprite: SpriteSheetBundle {
            transform: Transform {
                translation: position.translation(CHARACTER_Z, tile_size),
                ..default()
            },
            texture_atlas: textures.texture_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: config.actor_settings.sprite_index,
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
