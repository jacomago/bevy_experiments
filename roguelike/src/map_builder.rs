use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

enum Direction {
    Horizontal,
    Vertical,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while NUM_ROOMS > self.rooms.len() {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH),
                rng.range(1, SCREEN_HEIGHT),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if in_bounds(p) {
                        self.map.tiles[[p.y as usize, p.x as usize]] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn apply_tunnel(&mut self, inc1: usize, inc2: usize, x_or_y: usize, direction: Direction) {
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

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_tunnel(
                    prev.x as usize,
                    new.x as usize,
                    prev.y as usize,
                    Direction::Horizontal,
                );
                self.apply_tunnel(
                    prev.y as usize,
                    new.y as usize,
                    new.x as usize,
                    Direction::Vertical,
                );
            } else {
                self.apply_tunnel(
                    prev.x as usize,
                    new.x as usize,
                    new.y as usize,
                    Direction::Horizontal,
                );
                self.apply_tunnel(
                    prev.y as usize,
                    new.y as usize,
                    prev.x as usize,
                    Direction::Vertical,
                );
            }
        }
    }
}
