use super::{BuildingId, RoadId};

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
