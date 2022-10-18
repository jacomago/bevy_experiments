use bevy::prelude::*;

use bevy_turborand::rng::{Rng, TurboRand};
use bevy_turborand::{DelegatedRng, RngComponent};
use nannou_core::prelude::Rect;

use crate::components::map_position::MapPosition;
use crate::entities::TileType;

use self::standard::StandardArchitect;

use super::grid_map::DjikstraMapCalc;
use super::tile_map::{in_bounds, TileMap};

mod empty;
mod standard;

trait MapArchitect {
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder;
}

#[derive(Debug, Default)]
pub struct MapBuilder {
    pub map: TileMap,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<MapPosition>,
    pub player_start: MapPosition,
    pub winitem_start: MapPosition,
}

enum Direction {
    Horizontal,
    Vertical,
}

impl MapBuilder {
    pub fn new(mut rng: RngComponent, height: usize, width: usize) -> Self {
        let mut architect = StandardArchitect::new();
        architect.builder(height, width, &mut rng)
    }

    fn find_most_distant(&self) -> MapPosition {
        self.map.djikstra_map(&self.player_start).furthest_point()
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(
        &mut self,
        rng: &mut Rng,
        width: usize,
        height: usize,
        max_room_size: usize,
        num_rooms: usize,
    ) {
        while num_rooms > self.rooms.len() {
            let room = Rect::from_x_y_w_h(
                rng.usize(1..width) as f32,
                rng.usize(1..height) as f32,
                rng.usize(2..max_room_size) as f32,
                rng.usize(2..max_room_size) as f32,
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
                                if in_bounds(IVec2::from_array([x, y]), width, height) {
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

    fn build_corridors(&mut self, rng: &mut RngComponent) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| (a.xy().x as i32).cmp(&(b.xy().x as i32)));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].xy();
            let new = room.xy();

            if rng.usize(0..2) == 1 {
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
