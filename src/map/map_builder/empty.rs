use bevy_turborand::{DelegatedRng, RngComponent};

use crate::{components::map_position::MapPosition, entities::TileType, map::tile_map::TileMap};

use super::{MapArchitect, MapBuilder};

const NUM_MONSTERS: usize = 50;
pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn builder(&mut self, height: usize, width: usize, rng: &mut RngComponent) -> MapBuilder {
        let mut mb = MapBuilder {
            map: TileMap::new(height, width),
            monster_spawns: Vec::new(),
            player_start: MapPosition::ZERO,
            winitem_start: MapPosition::ZERO,
        };
        mb.fill(TileType::Floor);
        mb.player_start = MapPosition::new(
            (width / 2).try_into().unwrap(),
            (height / 2).try_into().unwrap(),
        );
        mb.winitem_start = mb.find_most_distant();
        mb.monster_spawns = (0..NUM_MONSTERS)
            .map(|_| MapPosition::new(rng.i32(1..width as i32), rng.i32(1..height as i32)))
            .collect();
        mb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let mut arch = EmptyArchitect {};
        let mut rng = RngComponent::new();
        let mb = arch.builder(10, 20, &mut rng);
        println!("{}", mb.map);
    }
}
