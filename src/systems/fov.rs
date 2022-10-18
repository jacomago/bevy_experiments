use bevy::{math::ivec2, prelude::*, utils::HashSet};
use nannou_core::prelude::PI;

use crate::{
    components::map_position::MapPosition,
    entities::{Player, Tile},
    map::{grid_graph::base_map::BaseMap, map_builder::MapBuilder, tile_map::TileMap},
};

pub fn fov(mut query: Query<(&MapPosition, &mut FieldOfView)>, map: Res<MapBuilder>) {
    query
        .iter_mut()
        .filter(|(_, f)| f.is_dirty)
        .for_each(|(p, mut f)| {
            f.update(p, &map.map);
        });
}

pub fn set_fov_visibility(
    player_fov: Query<(&FieldOfView, With<Player>)>,
    mut visibility_query: Query<(Entity, &mut Visibility, &MapPosition)>,
    mut tiles: Query<(&mut TextureAtlasSprite, With<Tile>)>,
) {
    let (fov, _) = player_fov.single();
    visibility_query.iter_mut().for_each(|(entity, mut v, p)| {
        if fov.visible_positions.contains(p) {
            v.is_visible = true;
            if let Ok((mut tile_sprite, _)) = tiles.get_mut(entity) {
                tile_sprite.color = Color::WHITE;
            }
        } else if let Ok((mut tile_sprite, _)) = tiles.get_mut(entity) {
            tile_sprite.color = Color::DARK_GRAY;
        } else {
            v.is_visible = false;
        }
    });
}

#[derive(Component, Default, Debug)]
pub struct FieldOfView {
    pub visible_positions: HashSet<MapPosition>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_positions: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_positions: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
    pub fn update(&mut self, p: &MapPosition, map: &TileMap) {
        self.visible_positions = field_of_view_set(p, self.radius, map);
        self.is_dirty = false;
    }
}

fn circle_set(radius: i32) -> HashSet<(i32, i32)> {
    let r_f32 = radius as f32;
    let min_r2 = (r_f32 - 0.25).powf(2.0);
    let max_r2 = (r_f32 + 0.25).powf(2.0);
    (0..((PI * r_f32 + 1.0) as usize))
        .map(|v| v as f32 * 2.0 / r_f32)
        .map(|v| (r_f32 * (v.sin()), r_f32 * (v.cos())))
        .flat_map(|(x, y)| {
            [
                (x.floor(), y.floor()),
                (x.floor(), y.ceil()),
                (x.ceil(), y.floor()),
                (x.ceil(), y.ceil()),
            ]
        })
        .map(|(x, y)| ((x, y), x.powf(2.0) + y.powf(2.0)))
        .filter(|(_, v)| *v > min_r2 && *v < max_r2)
        .map(|((x, y), _)| (x as i32, y as i32))
        .collect()
}

fn trace_path(
    p: &MapPosition,
    p2: &MapPosition,
    radius: i32,
    map: &TileMap,
) -> HashSet<MapPosition> {
    let scale_vector = (p2.position - p.position).as_vec2() / radius as f32;
    let mut res = HashSet::new();
    for p in (0..(radius + 1) as usize)
        .map(|i| p.position.as_vec2() + i as f32 * scale_vector)
        .map(|v| MapPosition {
            position: ivec2(v.x.round() as i32, v.y.round() as i32),
        })
    {
        res.insert(p);
        if !map.can_enter_tile(&p) {
            break;
        }
    }
    res
}

fn field_of_view_set(p: &MapPosition, radius: i32, map: &TileMap) -> HashSet<MapPosition> {
    // go through values of circle making a paht
    // adding to visible points on the way
    // if hit wall halt path
    let circle_set = circle_set(radius);
    circle_set
        .iter()
        .flat_map(|(x, y)| {
            let end_pos = MapPosition::from_ivec2(p.position + ivec2(*x, *y));
            trace_path(p, &end_pos, radius, map)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_set_low() {
        assert_eq!(
            circle_set(1),
            vec![(0, 1), (1, 0), (-1, 0), (0, -1)].into_iter().collect()
        );
    }

    #[test]
    fn test_circle_set_high() {
        assert_eq!(
            circle_set(4),
            vec![
                (0, 4),
                (3, -3),
                (-3, 3),
                (3, 3),
                (-4, 1),
                (-4, 0),
                (-1, 4),
                (0, -4),
                (4, 1),
                (-1, -4),
                (-3, -3),
                (1, 4),
                (4, 0),
                (1, -4),
                (4, -1),
                (-4, -1)
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn test_trace_path_basic() {
        let p = MapPosition::new(0, 0);
        let p2 = MapPosition::new(2, 0);
        let map = TileMap::new(10, 10);
        assert_eq!(
            trace_path(&p, &p2, 2, &map),
            vec![
                MapPosition::new(0, 0),
                MapPosition::new(1, 0),
                MapPosition::new(2, 0)
            ]
            .into_iter()
            .collect()
        );
    }
    #[test]
    fn test_trace_path_complex() {
        let p = MapPosition::new(1, 2);
        let p2 = MapPosition::new(5, 5);
        let map = TileMap::new(10, 10);
        assert_eq!(
            trace_path(&p, &p2, 5, &map),
            vec![
                MapPosition::new(1, 2),
                MapPosition::new(2, 3),
                MapPosition::new(4, 4),
                MapPosition::new(3, 4),
                MapPosition::new(3, 3),
                MapPosition::new(5, 5)
            ]
            .into_iter()
            .collect()
        );
    }
    #[test]
    fn test_trace_path_backwards() {
        let p = MapPosition::new(5, 5);
        let p2 = MapPosition::new(1, 5);
        let map = TileMap::new(10, 10);
        assert_eq!(
            trace_path(&p, &p2, 4, &map),
            vec![
                MapPosition::new(1, 5),
                MapPosition::new(2, 5),
                MapPosition::new(3, 5),
                MapPosition::new(4, 5),
                MapPosition::new(5, 5)
            ]
            .into_iter()
            .collect()
        );
    }
    #[test]
    fn test_field_of_view_set_simple() {
        let p = MapPosition::new(0, 0);
        let map = TileMap::new(10, 10);
        // In corner so no negatives other than walls
        assert_eq!(
            field_of_view_set(&p, 2, &map),
            vec![
                MapPosition::new(0, 0),
                MapPosition::new(1, 1),
                MapPosition::new(0, 1),
                MapPosition::new(-1, 0),
                MapPosition::new(0, 0),
                MapPosition::new(-1, -1),
                MapPosition::new(-1, 1),
                MapPosition::new(1, -1),
                MapPosition::new(0, -1),
                MapPosition::new(1, 0),
                MapPosition::new(2, 1),
                MapPosition::new(1, 2),
                MapPosition::new(0, 2),
                MapPosition::new(2, 0),
            ]
            .into_iter()
            .collect()
        );
    }
    #[test]
    fn test_field_of_view_set_not_centre() {
        let p = MapPosition::new(4, 3);
        let map = TileMap::new(10, 10);
        // In corner so no negatives
        assert_eq!(
            field_of_view_set(&p, 1, &map),
            vec![
                MapPosition::new(4, 3),
                MapPosition::new(3, 3),
                MapPosition::new(4, 4),
                MapPosition::new(5, 3),
                MapPosition::new(4, 2)
            ]
            .into_iter()
            .collect()
        );
    }
}
