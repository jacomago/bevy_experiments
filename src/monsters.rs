use crate::loading::TextureAtlasAssets;
use crate::map::map_builder::MapBuilder;
use crate::map::map_position::MapPosition;
use crate::player::Player;
use crate::GameState;

use bevy::prelude::*;

const MONSTER_SPRITE_INDEX: usize = 69;
const MONSTERS_Z: f32 = 1.;

pub struct MonstersPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_monsters))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(collisions));
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

pub fn collisions(
    mut commands: Commands,
    monsters: Query<(Entity, &MapPosition, With<Monster>)>,
    player_position_query: Query<&MapPosition, With<Player>>,
) {
    let player_position = player_position_query.single();
    monsters
        .iter()
        .filter(|(_, p, _)| *p == player_position)
        .for_each(|(e, _, _)| commands.entity(e).despawn_recursive());
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
