use std::fmt::Display;

use crate::components::map_position::MapPosition;
use crate::config::Architect;
use crate::entities::TileType;
use bevy::utils::HashSet;
use bevy_turborand::{DelegatedRng, RngComponent};

use self::automata::CellularAutomataArchitect;
use self::drunkard::DrunkardArchitect;
use self::empty::EmptyArchitect;
use self::prefab::apply_prefab;
use self::standard::StandardArchitect;

use super::grid_map::base_map::BaseMap;
use super::grid_map::DjikstraMapCalc;
use super::tile_map::TileMap;

mod automata;
mod drunkard;
mod empty;
mod prefab;
mod standard;

trait MapArchitect {
    fn monster_distance(&self) -> f32;
    fn num_monsters(&self) -> usize;
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder;

    fn monster_spawns(
        &self,
        start: &MapPosition,
        map: &TileMap,
        rng: &mut RngComponent,
    ) -> HashSet<MapPosition> {
        let tiles = map
            .tiles
            .indexed_iter()
            .map(|(idx, t)| (MapPosition::from_utuple(&idx), t))
            .filter(|(idx, t)| {
                **t == TileType::Floor && idx.distance(start) > self.monster_distance()
            })
            .map(|(idx, _)| idx)
            .collect::<Vec<MapPosition>>();

        let spawns = rng
            .sample_multiple(&tiles, self.num_monsters())
            .iter()
            .map(|f| **f)
            .collect();
        spawns
    }
}

#[derive(Debug, Default)]
pub struct MapBuilder {
    pub map: TileMap,
    pub monster_spawns: HashSet<MapPosition>,
    pub player_start: MapPosition,
    pub winitem_start: MapPosition,
}

fn pick_architect(architect: &Architect) -> Box<dyn MapArchitect> {
    match architect {
        Architect::Empty => Box::new(EmptyArchitect::new()),
        Architect::Standard => Box::new(StandardArchitect::new()),
        Architect::Automata => Box::new(CellularAutomataArchitect::new()),
        Architect::Drunkard => Box::new(DrunkardArchitect::new()),
    }
}

impl MapBuilder {
    pub fn new(mut rng: RngComponent, height: usize, width: usize, architect: &Architect) -> Self {
        let mut map_arch = pick_architect(architect);
        let mut mb = map_arch.builder(height, width, &mut rng);
        const MAX_ATTEMPTS: usize = 10;
        apply_prefab(&mut mb, MAX_ATTEMPTS, &mut rng, 20, 2000);
        mb
    }

    fn find_most_distant(&self) -> MapPosition {
        self.map.djikstra_map(&self.player_start).furthest_point()
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn fill_in_unreachable(&mut self) {
        self.map
            .djikstra_map(&self.player_start)
            .far_points(None)
            .iter()
            .for_each(|p| {
                self.map.set(p, TileType::Wall);
            });
    }
}

impl Display for MapBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_tiles = self
            .map
            .tiles
            .rows()
            .into_iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(y, tile)| {
                        let mp = MapPosition::from_utuple(&(x, y));
                        if self.player_start == mp {
                            "@".to_string()
                        } else if self.winitem_start == mp {
                            "?".to_string()
                        } else if self.monster_spawns.contains(&mp) {
                            "M".to_string()
                        } else {
                            format!("{}", tile)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        f.write_fmt(format_args!("{}", str_tiles))
    }
}
