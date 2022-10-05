use crate::map::map_builder::MapBuilder;
use crate::map::map_position::MapPosition;
use crate::stages::{end_turn, TurnState};
use crate::systems::movement::movement;
use crate::systems::movement::WantsToMove;
use crate::GameState;
use crate::{loading::TextureAtlasAssets, stages::GameStage};

use bevy::{math::ivec2, prelude::*};
use iyes_loopless::prelude::ConditionSet;
use rand::{thread_rng, Rng};

const MONSTER_SPRITE_INDEX: usize = 69;
const MONSTERS_Z: f32 = 1.;

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
    pub position: MapPosition,
    #[bundle]
    sprite: SpriteSheetBundle,
}

fn spawn_monsters(
    mut commands: Commands,
    textures: Res<TextureAtlasAssets>,
    map_builder: Res<MapBuilder>,
) {
    map_builder.rooms.iter().skip(1).for_each(|room| {
        let position = MapPosition::new(room.x() as i32, room.y() as i32);
        commands.spawn_bundle(MonsterBundle {
            position,
            sprite: SpriteSheetBundle {
                transform: Transform {
                    translation: position.translation(MONSTERS_Z),
                    ..default()
                },
                texture_atlas: textures.texture_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: MONSTER_SPRITE_INDEX,
                    ..default()
                },
                ..default()
            },
            ..default()
        });
    });
}

fn random_move(
    monsters: Query<(Entity, &MapPosition, With<Monster>)>,
    mut move_events: EventWriter<WantsToMove>,
) {
    let mut rng = thread_rng();
    monsters.iter().for_each(|(entity, p, _)| {
        let destination = MapPosition::from_ivec2(
            match rng.gen_range(0..4) {
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
