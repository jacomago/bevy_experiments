use bevy::prelude::*;
use ndarray::{Array, Ix2};
use std::fmt::Display;
use std::usize;

use crate::{components::map_position::MapPosition, entities::TileType};

use super::grid_map::{base_map::BaseMap, DjikstraMapCalc};

#[derive(Default, Debug)]
pub struct TileMap {
    pub height: usize,
    pub width: usize,
    pub tiles: Array<TileType, Ix2>,
}

pub fn in_bounds(point: IVec2, width: usize, height: usize) -> bool {
    point.x >= 0
        && width > point.x.try_into().unwrap()
        && point.y >= 0
        && height > point.y.try_into().unwrap()
}

impl BaseMap for TileMap {
    type Output = TileType;
    fn can_enter_tile(&self, point: &MapPosition) -> bool {
        self.in_bounds(point.position)
            && self
                .tiles
                .get(point.as_utuple())
                .map_or(false, |&s| s == TileType::Floor)
    }
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    fn value(&self, p: &MapPosition) -> Self::Output {
        self.tiles[p.as_utuple()]
    }
}

impl DjikstraMapCalc for TileMap {}

impl Display for TileMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_tiles = self
            .tiles
            .rows()
            .into_iter()
            .map(|row| {
                row.iter()
                    .map(|tile| format!("{}", tile))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_fmt(format_args!("{}", str_tiles))
    }
}

impl TileMap {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            tiles: Array::<TileType, Ix2>::from_elem((height, width), TileType::Floor),
        }
    }

    pub fn in_bounds(&self, point: IVec2) -> bool {
        in_bounds(point, self.width, self.height)
    }
}

#[test]
fn test_djikstra() {
    let map = TileMap::new(10, 20);

    let start = MapPosition::new(0, 0);
    let dmap = map.djikstra_map(&start);
    assert_eq!(dmap.value(&MapPosition::new(1, 1)), Some(2));
}
