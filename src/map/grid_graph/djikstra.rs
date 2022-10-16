use crate::map::map_position::MapPosition;

use super::{djikstra_map::DjikstraMap, neighbours::Neighbours};

pub trait DjikstraMapCalc: Neighbours {
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn djikstra_map(&self, start_node: &MapPosition) -> DjikstraMap {
        let mut dmap = DjikstraMap::new(self.height(), self.width(), start_node.as_utuple());

        let mut frontier: Vec<MapPosition> = vec![*start_node];

        while !frontier.is_empty() {
            let mut new_frontier: Vec<MapPosition> = vec![];

            frontier.iter().for_each(|f| {
                self.neighbours(f).iter().for_each(|n| {
                    if dmap.value(n).is_none() {
                        dmap.set(n, dmap.value(f).unwrap() + 1);
                        new_frontier.push(*n);
                    }
                });
            });
            frontier = new_frontier;
        }
        dmap
    }
}
