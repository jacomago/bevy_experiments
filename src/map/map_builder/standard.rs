use bevy::{prelude::*, utils::HashSet};
use bevy_turborand::{
    rng::{Rng, TurboRand},
    DelegatedRng, RngComponent,
};
use nannou_core::prelude::Rect;

use crate::{
    components::map_position::MapPosition,
    entities::TileType,
    map::{
        grid_map::DjikstraMapCalc,
        tile_map::{in_bounds, TileMap},
    },
};

use super::{MapArchitect, MapBuilder};

enum Direction {
    Horizontal,
    Vertical,
}

pub struct StandardArchitect {
    max_room_size: usize,
    num_rooms: usize,
    rooms: Vec<Rect>,
}

impl MapArchitect for StandardArchitect {
    fn entity_distance(&self) -> f32 {
        0.0
    }
    fn num_monsters(&self) -> usize {
        self.num_rooms
    }
    fn num_items(&self) -> usize {
        self.num_rooms
    }
    fn entity_spawns(
        &self,
        _: &MapPosition,
        _: &TileMap,
        _: &mut RngComponent,
    ) -> HashSet<MapPosition> {
        self.rooms
            .iter()
            .skip(1)
            .map(|room| MapPosition::new(room.x() as i32, room.y() as i32))
            .collect()
    }
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            player_start: MapPosition::default(),
            ..default()
        };
        mb.fill(TileType::Wall);
        self.rooms = self.build_random_rooms(
            &mut mb.map,
            rng.get_mut(),
            width,
            height,
            self.max_room_size,
            self.num_rooms,
        );
        self.build_corridors(&self.rooms.clone(), &mut mb.map, rng);
        mb.monster_spawns = self.entity_spawns(&MapPosition::ZERO, &mb.map, rng);
        mb.item_spawns = self.entity_spawns(&MapPosition::ZERO, &mb.map, rng);
        let dmap = mb.map.djikstra_map(&MapPosition::new(
            self.rooms[0].x() as i32,
            self.rooms[0].y() as i32,
        ));
        let longest_path = dmap.calculate_longest_path();
        mb.player_start = longest_path[0];
        mb.winitem_start = *longest_path.last().unwrap();
        mb
    }
}

impl StandardArchitect {
    pub fn new(_num_monsters: usize, num_items: usize, entity_distance: f32) -> Self {
        Self {
            max_room_size: entity_distance as usize,
            num_rooms: num_items,
            rooms: Vec::new(),
        }
    }
    fn build_random_rooms(
        &mut self,
        map: &mut TileMap,
        rng: &mut Rng,
        width: usize,
        height: usize,
        max_room_size: usize,
        num_rooms: usize,
    ) -> Vec<Rect> {
        let mut rooms: Vec<Rect> = Vec::new();
        while num_rooms > rooms.len() {
            let room = Rect::from_x_y_w_h(
                rng.usize(1..width) as f32,
                rng.usize(1..height) as f32,
                rng.usize(2..max_room_size) as f32,
                rng.usize(2..max_room_size) as f32,
            );
            let mut overlap = false;
            for r in &rooms {
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
                                    map.tiles[[y as usize, x as usize]] = TileType::Floor;
                                }
                            });
                    });

                rooms.push(room);
            }
        }

        rooms
    }

    fn apply_tunnel(
        &mut self,
        map: &mut TileMap,
        inc1: usize,
        inc2: usize,
        x_or_y: usize,
        direction: &Direction,
    ) {
        use std::cmp::{max, min};
        for inc in min(inc1, inc2)..max(inc1, inc2) {
            let pair = match direction {
                Direction::Horizontal => (x_or_y, inc),
                Direction::Vertical => (inc, x_or_y),
            };
            if let Some(tile) = map.tiles.get_mut(pair) {
                *tile = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, in_rooms: &[Rect], map: &mut TileMap, rng: &mut RngComponent) {
        let mut rooms = Vec::from(in_rooms);
        rooms.sort_by(|a, b| (a.xy().x as i32).cmp(&(b.xy().x as i32)));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].xy();
            let new = room.xy();

            if rng.usize(0..2) == 1 {
                self.apply_tunnel(
                    map,
                    prev.x as usize,
                    new.x as usize,
                    prev.y as usize,
                    &Direction::Horizontal,
                );
                self.apply_tunnel(
                    map,
                    prev.y as usize,
                    new.y as usize,
                    new.x as usize,
                    &Direction::Vertical,
                );
            } else {
                self.apply_tunnel(
                    map,
                    prev.x as usize,
                    new.x as usize,
                    new.y as usize,
                    &Direction::Horizontal,
                );
                self.apply_tunnel(
                    map,
                    prev.y as usize,
                    new.y as usize,
                    prev.x as usize,
                    &Direction::Vertical,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let mut arch = StandardArchitect::new(50, 20, 10.0);
        let mut rng = RngComponent::new();
        let mb = arch.builder(40, 80, &mut rng);
        println!("{}", mb);
    }
}
