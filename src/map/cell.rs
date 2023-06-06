use crate::direction::CardinalDirection;

use super::{Building, BuildingId, Road, RoadId, RoadOrientation};
use grid::Grid as InnerGrid;
use iter_tools::Itertools;
use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Road(RoadId),
    Building(BuildingId),
}

impl Cell {
    pub fn is_road(&self) -> bool {
        matches!(self, Cell::Road(_))
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}
