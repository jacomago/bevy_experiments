use bevy::prelude::*;

use bevy_turborand::rng::{Rng, TurboRand};
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use nannou_core::prelude::Rect;

use super::grid_graph::DjikstraMapCalc;
use super::map_position::MapPosition;
use super::tile::TileType;
use super::tile_map::{in_bounds, TileMap};
use super::{MAP_HEIGHT, MAP_WIDTH, MAX_ROOM_SIZE};

const NUM_ROOMS: usize = 20;

#[derive(Debug, Default)]
pub struct MapBuilder {
    pub map: TileMap,
    pub rooms: Vec<Rect>,
    pub player_start: MapPosition,
    pub winitem_start: MapPosition,
    pub rng: RngComponent,
}

enum Direction {
    Horizontal,
    Vertical,
}

pub fn insert_mapbuilder(mut commands: Commands, mut rng: ResMut<GlobalRng>) {
    commands.insert_resource(MapBuilder::new(
        RngComponent::from(&mut rng),
        MAP_HEIGHT,
        MAP_WIDTH,
        MAX_ROOM_SIZE,
    ));
}

impl MapBuilder {
    pub fn new(mut rng: RngComponent, height: usize, width: usize, max_room_size: usize) -> Self {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            rooms: Vec::new(),
            player_start: MapPosition::default(),
            ..default()
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng.get_mut(), width, height, max_room_size);
        mb.build_corridors(&mut rng);
        let dmap = mb.map.djikstra_map(&MapPosition::new(
            mb.rooms[0].x() as i32,
            mb.rooms[0].y() as i32,
        ));
        let longest_path = dmap.calculate_longest_path();
        mb.player_start = longest_path[0];
        mb.winitem_start = *longest_path.last().unwrap();
        mb.rng = rng;
        mb
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
    ) {
        while NUM_ROOMS > self.rooms.len() {
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
