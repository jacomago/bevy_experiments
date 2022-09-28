use std::usize;

use ndarray::{Array, Dim, Ix2};

use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Array<TileType, Ix2>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: Array::<TileType, Ix2>::from_elem(
                (SCREEN_HEIGHT, SCREEN_WIDTH),
                TileType::Floor,
            ),
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0
            && SCREEN_WIDTH > point.x.try_into().unwrap()
            && point.y >= 0
            && SCREEN_HEIGHT > point.y.try_into().unwrap()
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && self
                .tiles
                .get((point.x as usize, point.y as usize))
                .map(|&s| s == TileType::Floor)
                .unwrap_or(false)
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                match self.tiles.get((y, x)) {
                    Some(t) => match t {
                        TileType::Floor => {
                            ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                        }
                    },
                    None => {}
                }
            }
        }
    }
}
