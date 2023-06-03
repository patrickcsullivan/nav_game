mod builder;
mod read;

pub use builder::{Builder, BuilderError};
pub use read::{from_csvs, ReadError};

use super::{Building, Road};
use vek::Vec2;

/// A grid representing the map of the game world.
///
/// Roads can exist along the lines of the grid, and buildings can exist
/// within cells of the grid.
#[derive(Debug, Clone)]
pub struct MapGrid {
    /// Width and height of the map.
    dim: Vec2<usize>,

    /// Roads.
    ///
    /// The roads may intersect.
    roads: Vec<Road>,

    /// Buildings.
    buildings: Vec<Building>,
}

impl MapGrid {
    /// Returns the buildings in the map.
    pub fn buildings(&self) -> &[Building] {
        &self.buildings
    }

    /// Returns the roads in the map.
    pub fn roads(&self) -> &[Road] {
        &self.roads
    }
}
