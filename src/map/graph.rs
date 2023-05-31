use vek::Vec2;

use super::grid::Building;

pub struct BuildingNode {
    building: Building,
}

pub struct RoadCoordNode {
    coord: Vec2<usize>,
    north: Option<Vec2<usize>>,
    east: Option<Vec2<usize>>,
    south: Option<Vec2<usize>>,
    west: Option<Vec2<usize>>,
}
// /// A grid representing the map of the game world.
// ///
// /// Streets can exist along the lines of the grid, and buildings can exist
// /// within cells of the grid.
// pub struct MapGrid {
//     /// Width and height of the map.
//     dim: Vec2<usize>,

//     /// Streets.
//     ///
//     /// The streets may intersect.
//     streets: Vec<Street>,
// }

// pub enum StreetPoint {
//     Intersection(StreetIntersection),
//     Terminus(StreetTerminus),
// }

// /// An intersection between multiple roads.
// pub struct StreetIntersection {
//     location: Vec2<usize>,

//     neighbors: Vec<&Street
// }

// /// A building.
// pub struct Building {
//     /// Origin of the building.
//     origin: Vec2<usize>,

//     /// Width and height of the building.
//     dim: Vec2<usize>,

//     /// Name of the building.
//     name: Option<String>,
// }
