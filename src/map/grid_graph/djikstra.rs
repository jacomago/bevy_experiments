use crate::map::map_position::MapPosition;

use super::{base_map::BaseMap, djikstra_map::DjikstraMap};

pub trait DjikstraMapCalc: BaseMap {
    fn depth_djikstra_map(&self, start_node: &MapPosition, max_depth: Option<i32>) -> DjikstraMap {
        let mut dmap = DjikstraMap::new(self.height(), self.width(), start_node.as_utuple());

        let mut frontier: Vec<MapPosition> = vec![*start_node];

        while !frontier.is_empty() {
            let mut new_frontier: Vec<MapPosition> = vec![];

            frontier.iter().for_each(|f| {
                let f_value = dmap.value(f).unwrap();
                if max_depth.is_none() || f_value <= max_depth.unwrap() {
                    self.neighbours(f).iter().for_each(|n| {
                        if dmap.value(n).is_none() {
                            dmap.set(n, f_value + 1);
                            new_frontier.push(*n);
                        }
                    });
                }
            });
            frontier = new_frontier;
        }
        dmap
    }
    fn djikstra_map(&self, start_node: &MapPosition) -> DjikstraMap {
        self.depth_djikstra_map(start_node, None)
    }
}
