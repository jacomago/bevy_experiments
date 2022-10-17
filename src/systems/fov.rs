use bevy::{prelude::*, utils::HashSet};
use nannou_core::prelude::PI;

use crate::map::{
    grid_graph::neighbours::Neighbours, map_builder::MapBuilder, map_position::MapPosition,
    tile_map::TileMap,
};

pub struct FOVPlugin;

impl Plugin for FOVPlugin {
    fn build(&self, _: &mut App) {}
}

pub fn fov(mut query: Query<(&MapPosition, &mut FieldOfView)>, map: Res<MapBuilder>) {
    query
        .iter_mut()
        .filter(|(_, f)| f.is_dirty)
        .for_each(|(p, mut f)| {
            f.visible_positions = field_of_view_set(p, f.radius, &map);
            f.is_dirty = false;
        });
}

fn circle_set(radius: i32) -> Vec<(i32, i32)> {
    let r_f32 = radius as f32;
    let r2 = r_f32.powf(2.0);
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
        .filter(|(_, v)| *v > r2 - 1.0 && *v < r2 + 1.0)
        .map(|((x, y), _)| (x as i32, y as i32))
        .collect()
}

fn trace_path(
    p: &MapPosition,
    p2: &MapPosition,
    radius: i32,
    map: &TileMap,
) -> HashSet<MapPosition> {
    let scale_vector = p2.position - p.position;
    let mut res = HashSet::new();
    for p in (0..radius as usize)
        .map(|i| p.position + i as i32 * scale_vector)
        .map(|v| MapPosition { position: v })
    {
        if map.can_enter_tile(&p) {
            res.insert(p);
        } else {
            break;
        }
    }
    res
}

fn field_of_view_set(p: &MapPosition, radius: i32, map: &MapBuilder) -> HashSet<MapPosition> {
    // go through values of circle making a paht
    // adding to visible points on the way
    // if hit wall halt path
    let circle_set = circle_set(radius);
    circle_set
        .iter()
        .map(|(x, y)| trace_path(p, &MapPosition::new(*x, *y), radius, &map.map))
        .flatten()
        .collect()
}

#[derive(Component, Default)]
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
}
