use std::usize;

use ndarray::{Array, Ix2};

use crate::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Array<TileType, Ix2>,
}

pub fn in_bounds(point: Point) -> bool {
    point.x >= 0
        && SCREEN_WIDTH > point.x.try_into().unwrap()
        && point.y >= 0
        && SCREEN_HEIGHT > point.y.try_into().unwrap()
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

    pub fn can_enter_tile(&self, point: Point) -> bool {
        in_bounds(point)
            && self
                .tiles
                .get((point.y as usize, point.x as usize))
                .map(|&s| s == TileType::Floor)
                .unwrap_or(false)
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                match self.tiles.get((y as usize, x as usize)) {
                    Some(t) => match t {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                YELLOW,
                                BLACK,
                                to_cp437('.'),
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                GREEN,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    },
                    None => {}
                }
            }
        }
    }
}
