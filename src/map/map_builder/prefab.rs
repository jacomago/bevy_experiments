use bevy::prelude::info;
use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{
    components::map_position::MapPosition,
    entities::TileType,
    map::grid_map::{base_map::BaseMap, DjikstraMapCalc},
};

use super::MapBuilder;

#[derive(Debug, Clone)]
struct Fortress {
    input: String,
    height: usize,
    width: usize,
}

impl Default for Fortress {
    fn default() -> Self {
        Self {
            input: "
        .........
        ..#####..
        ..#...#..
        ..#.M.#..
        .##...##.
        .M.....M.
        .##...##.
        ..#...#..
        ..#.M.#..
        ..#####..
        .........
        "
            .to_string(),
            height: 8,
            width: 11,
        }
    }
}

pub fn apply_prefab(
    map_builder: &mut MapBuilder,
    max_attempts: usize,
    rng: &mut RngComponent,
    min: i32,
    max: i32,
) {
    let fortress = Fortress::default();
    let mut placed = false;
    let dmap = map_builder.map.djikstra_map(&map_builder.player_start);

    let mut attempts = 0;
    while !placed && attempts < max_attempts {
        let poss_rect = (
            rng.usize(0..(dmap.width() - fortress.width)),
            rng.usize(0..(dmap.height() - fortress.height)),
        );
        let mut can_place = false;
        (poss_rect.0..poss_rect.0 + fortress.width).for_each(|x| {
            (poss_rect.1..poss_rect.1 + fortress.height).for_each(|y| {
                let mp = MapPosition::from_utuple(&(y, x));
                let distance = dmap.value(&mp);
                if distance.is_some()
                    && distance.map(|v| v > min && v < max).unwrap()
                    && map_builder.winitem_start != mp
                {
                    can_place = true;
                }
            })
        });
        if can_place {
            place_fortress(map_builder, &fortress, &poss_rect);
            placed = true;
        }
        attempts += 1;
    }
}

fn place_fortress(map_builder: &mut MapBuilder, fortress: &Fortress, placement: &(usize, usize)) {
    fortress
        .input
        .trim()
        .split("\n")
        .into_iter()
        .enumerate()
        .for_each(|(y, s)| {
            s.trim()
                .split("")
                .into_iter()
                .enumerate()
                .for_each(|(x, cs)| {
                    let p = MapPosition::from_utuple(&(y + placement.1, x + placement.0));
                    map_builder.monster_spawns.remove(&p);
                    match cs {
                        "." => map_builder.map.set(&p, TileType::Floor),
                        "M" => {
                            map_builder.map.set(&p, TileType::Floor);
                            map_builder.monster_spawns.insert(p);
                        }
                        "#" => map_builder.map.set(&p, TileType::Wall),
                        _ => info!("nothing to match on {}", cs),
                    };
                });
        });
}
