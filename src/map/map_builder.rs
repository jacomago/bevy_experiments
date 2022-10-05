use bevy::prelude::*;

use nannou_core::prelude::Rect;
use rand::thread_rng;
use rand::{rngs::ThreadRng, Rng};

use super::map_position::MapPosition;
use super::tile_map::{in_bounds, TileMap, TileType, MAP_HEIGHT, MAP_WIDTH};

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: TileMap,
    pub rooms: Vec<Rect>,
    pub player_start: MapPosition,
}

enum Direction {
    Horizontal,
    Vertical,
}

impl MapBuilder {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut mb = MapBuilder {
            map: TileMap::new(),
            rooms: Vec::new(),
            player_start: MapPosition::default(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(&mut rng);
        mb.build_corridors(&mut rng);
        mb.player_start = MapPosition::new(mb.rooms[0].x() as i32, mb.rooms[0].y() as i32);
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut ThreadRng) {
        while NUM_ROOMS > self.rooms.len() {
            let room = Rect::from_x_y_w_h(
                rng.gen_range(1..MAP_WIDTH) as f32,
                rng.gen_range(1..MAP_HEIGHT) as f32,
                rng.gen_range(2..10) as f32,
                rng.gen_range(2..10) as f32,
            );
            let mut overlap = false;
            for r in &self.rooms {
                if r.overlap(room).is_some() {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                (room.left() as i32..room.right() as i32)
                    .into_iter()
                    .for_each(|x| {
                        (room.bottom() as i32..room.top() as i32)
                            .into_iter()
                            .for_each(|y| {
                                if in_bounds(IVec2::from_array([x, y])) {
                                    self.map.tiles[[y as usize, x as usize]] = TileType::Floor;
                                }
                            });
                    });

                self.rooms.push(room);
            }
        }
    }

    fn apply_tunnel(&mut self, inc1: usize, inc2: usize, x_or_y: usize, direction: &Direction) {
        use std::cmp::{max, min};
        for inc in min(inc1, inc2)..max(inc1, inc2) {
            let pair = match direction {
                Direction::Horizontal => (x_or_y, inc),
                Direction::Vertical => (inc, x_or_y),
            };
            if let Some(tile) = self.map.tiles.get_mut(pair) {
                *tile = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut ThreadRng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| (a.xy().x as i32).cmp(&(b.xy().x as i32)));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].xy();
            let new = room.xy();

            if rng.gen_range(0..2) == 1 {
                self.apply_tunnel(
                    prev.x as usize,
                    new.x as usize,
                    prev.y as usize,
                    &Direction::Horizontal,
                );
                self.apply_tunnel(
                    prev.y as usize,
                    new.y as usize,
                    new.x as usize,
                    &Direction::Vertical,
                );
            } else {
                self.apply_tunnel(
                    prev.x as usize,
                    new.x as usize,
                    new.y as usize,
                    &Direction::Horizontal,
                );
                self.apply_tunnel(
                    prev.y as usize,
                    new.y as usize,
                    prev.x as usize,
                    &Direction::Vertical,
                );
            }
        }
    }
}
