use crate::prelude::*;

#[derive(Component, Default, PartialEq, Eq)]
pub struct MapPosition {
    pub position: IVec2,
}
