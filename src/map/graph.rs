use std::collections::HashMap;

use super::{Building, CoordBuildingAdjacency, MapGrid};
use iter_tools::Itertools;
use vek::Vec2;

/// A node in the graph that represents some building.
#[derive(Debug, Clone)]
pub struct BuildingNode {
    building: Building,
}

impl BuildingNode {
    /// Returns a new building node.
    pub fn new(building: Building) -> Self {
        Self { building }
    }

    /// Returns the building inside the node.
    pub fn building(&self) -> &Building {
        &self.building
    }
}

/// A node in the graph that represents a coordinate along some road.
#[derive(Debug, Clone, Copy)]
pub struct RoadCoordNode {
    coord: Vec2<usize>,
    north: Option<Vec2<usize>>,
    east: Option<Vec2<usize>>,
    south: Option<Vec2<usize>>,
    west: Option<Vec2<usize>>,
    northeast: Option<usize>,
    southeast: Option<usize>,
    southwest: Option<usize>,
    northwest: Option<usize>,
}

impl RoadCoordNode {
    /// Returns a new road coordinate node.
    pub fn new(coord: Vec2<usize>) -> Self {
        Self {
            coord,
            north: None,
            east: None,
            south: None,
            west: None,
            northeast: None,
            southeast: None,
            southwest: None,
            northwest: None,
        }
    }

    /// Set the specified building as adjacent to the road coordinate node in
    /// the given directions.
    fn set_building_adjacencies(&mut self, building_id: usize, adj: CoordBuildingAdjacency) {
        match adj {
            CoordBuildingAdjacency::Northeast => {
                self.northeast = Some(building_id);
            }
            CoordBuildingAdjacency::Southeast => {
                self.southeast = Some(building_id);
            }
            CoordBuildingAdjacency::Southwest => {
                self.southwest = Some(building_id);
            }
            CoordBuildingAdjacency::Northwest => {
                self.northwest = Some(building_id);
            }
            CoordBuildingAdjacency::NorthwestNortheast => {
                self.northwest = Some(building_id);
                self.northeast = Some(building_id);
            }
            CoordBuildingAdjacency::NortheastSouthEast => {
                self.northeast = Some(building_id);
                self.southeast = Some(building_id);
            }
            CoordBuildingAdjacency::SoutheastSouthWest => {
                self.southeast = Some(building_id);
                self.southwest = Some(building_id);
            }
            CoordBuildingAdjacency::SouthwestNorthwest => {
                self.southwest = Some(building_id);
                self.northwest = Some(building_id);
            }
        }
    }
}

/// A graph that connects coordinates along roads to each other and to buildings
/// in the game map.
#[derive(Debug, Clone)]
pub struct MapGraph {
    building_nodes: Vec<BuildingNode>,
    road_coord_nodes: HashMap<Vec2<usize>, RoadCoordNode>,
}

impl MapGraph {
    /// Constructs and returns a graph from the roads and buildings in the given
    /// `MapGrid`.
    pub fn new(grid: &MapGrid) -> Self {
        let mut graph = Self {
            building_nodes: vec![],
            road_coord_nodes: HashMap::new(),
        };

        let building_nodes = grid
            .buildings()
            .iter()
            .map(|b| BuildingNode::new(b.clone()))
            .collect_vec();

        // Add every coordinate that is adjacent to a building.
        for r in grid.roads() {
            for b in building_nodes.iter().map(|node| node.building()) {
                for (coord, adj) in b.get_connections(r) {
                    let coord_node = graph.get_or_create_road_coord(coord);
                    coord_node.set_building_adjacencies(b.id(), adj);
                }
            }
        }

        // Add every road origin and terminus.
        // for r in grid.road() {}

        graph.building_nodes = building_nodes;
        graph
    }

    /// Returns the building node with the given ID.
    pub fn get_building(&self, id: usize) -> Option<&BuildingNode> {
        // It happens to the be case that a building's ID corresponds to its
        // index in the list of buildings so we can simply access it by index.
        self.building_nodes.get(id)
    }

    /// Returns the road coordinate node with the given coordinates.
    pub fn get_road_cooord(&self, coord: Vec2<usize>) -> Option<&RoadCoordNode> {
        self.road_coord_nodes.get(&coord)
    }

    /// Gets the road coordinate node with the given coordinates or creates and
    /// adds a new node with the coordinates to the graph if one does not exist
    /// yet.
    fn get_or_create_road_coord(&mut self, coord: Vec2<usize>) -> &mut RoadCoordNode {
        self.road_coord_nodes
            .entry(coord)
            .or_insert_with(|| RoadCoordNode::new(coord))
    }
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

// fn m() {
//     MapGraph {
//         building_nodes: [
//             BuildingNode { building: Building { id: 0, origin: Vec2 { x: 2, y: 4 }, dim: Vec2 { x: 1, y: 1 }, name: Some("la casa") } }, BuildingNode { building: Building { id: 1, origin: Vec2 { x: 2, y: 5 }, dim: Vec2 { x: 1, y: 1 }, name: Some("la tienda") } }, BuildingNode { building: Building { id: 2, origin: Vec2 { x: 4, y: 1 }, dim: Vec2 { x: 1, y: 1 }, name: Some("el hospital") } }, BuildingNode { building: Building { id: 3, origin: Vec2 { x: 4, y: 6 }, dim: Vec2 { x: 4, y: 1 }, name: Some("el parque") } }, BuildingNode { building: Building { id: 4, origin: Vec2 { x: 6, y: 2 }, dim: Vec2 { x: 2, y: 1 }, name: Some("el supermercado") } }, BuildingNode { building: Building { id: 5, origin: Vec2 { x: 8, y: 3 }, dim: Vec2 { x: 2, y: 2 }, name: None } }],
//         road_coord_nodes: {
//             Vec2 { x: 2, y: 5 }: RoadCoordNode { coord: Vec2 { x: 2, y: 5 }, north: None, east: None, south: None, west: None, northeast: Some(1), southeast: Some(0), southwest: None, northwest: None },
//             Vec2 { x: 4, y: 2 }: RoadCoordNode { coord: Vec2 { x: 4, y: 2 }, north: None, east: None, south: None, west: None, northeast: None, southeast: Some(2), southwest: None, northwest: None },
//             Vec2 { x: 6, y: 3 }: RoadCoordNode { coord: Vec2 { x: 6, y: 3 }, north: None, east: None, south: None, west: None, northeast: Some(4), southeast: Some(4), southwest: None, northwest: None },
//             Vec2 { x: 8, y: 3 }: RoadCoordNode { coord: Vec2 { x: 8, y: 3 }, north: None, east: None, south: None, west: None, northeast: Some(5), southeast: None, southwest: Some(4), northwest: None },
//             Vec2 { x: 7, y: 3 }: RoadCoordNode { coord: Vec2 { x: 7, y: 3 }, north: None, east: None, south: None, west: None, northeast: None, southeast: Some(4), southwest: Some(4), northwest: None },
//             Vec2 { x: 10, y: 3 }: RoadCoordNode { coord: Vec2 { x: 10, y: 3 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: None, northwest: Some(5) },
//             Vec2 { x: 4, y: 6 }: RoadCoordNode { coord: Vec2 { x: 4, y: 6 }, north: None, east: None, south: None, west: None, northeast: Some(3), southeast: None, southwest: None, northwest: None }, Vec2 { x: 5, y: 2 }: RoadCoordNode { coord: Vec2 { x: 5, y: 2 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: Some(2), northwest: Some(2) }, Vec2 { x: 2, y: 4 }: RoadCoordNode { coord: Vec2 { x: 2, y: 4 }, north: None, east: None, south: None, west: None, northeast: Some(0), southeast: None, southwest: None, northwest: None }, Vec2 { x: 2, y: 6 }: RoadCoordNode { coord: Vec2 { x: 2, y: 6 }, north: None, east: None, south: None, west: None, northeast: None, southeast: Some(1), southwest: None, northwest: None }, Vec2 { x: 8, y: 6 }: RoadCoordNode { coord: Vec2 { x: 8, y: 6 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: None, northwest: Some(3) }, Vec2 { x: 3, y: 5 }: RoadCoordNode { coord: Vec2 { x: 3, y: 5 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: Some(0), northwest: Some(1) }, Vec2 { x: 10, y: 5 }: RoadCoordNode { coord: Vec2 { x: 10, y: 5 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: Some(5), northwest: None }, Vec2 { x: 8, y: 4 }: RoadCoordNode { coord: Vec2 { x: 8, y: 4 }, north: None, east: None, south: None, west: None, northeast: Some(5), southeast: Some(5), southwest: None, northwest: None }, Vec2 { x: 8, y: 5 }: RoadCoordNode { coord: Vec2 { x: 8, y: 5 }, north: None, east: None, south: None, west: None, northeast: None, southeast: Some(5), southwest: None, northwest: None }, Vec2 { x: 3, y: 6 }: RoadCoordNode { coord: Vec2 { x: 3, y: 6 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: Some(1), northwest: None }, Vec2 { x: 8, y: 7 }: RoadCoordNode { coord: Vec2 { x: 8, y: 7 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: Some(3), northwest: None }, Vec2 { x: 8, y: 2 }: RoadCoordNode { coord: Vec2 { x: 8, y: 2 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: None, northwest: Some(4) }, Vec2 { x: 4, y: 1 }: RoadCoordNode { coord: Vec2 { x: 4, y: 1 }, north: None, east: None, south: None, west: None, northeast: Some(2), southeast: None, southwest: None, northwest: None }, Vec2 { x: 9, y: 5 }: RoadCoordNode { coord: Vec2 { x: 9, y: 5 }, north: None, east: None, south: None, west: None, northeast: None, southeast: Some(5), southwest: Some(5), northwest: None }, Vec2 { x: 6, y: 6 }: RoadCoordNode { coord: Vec2 { x: 6, y: 6 }, north: None, east: None, south: None, west: None, northeast: Some(3), southeast: None, southwest: None, northwest: Some(3) }, Vec2 { x: 9, y: 3 }: RoadCoordNode { coord: Vec2 { x: 9, y: 3 }, north: None, east: None, south: None, west: None, northeast: Some(5), southeast: None, southwest: None, northwest: Some(5) }, Vec2 { x: 7, y: 6 }: RoadCoordNode { coord: Vec2 { x: 7, y: 6 }, north: None, east: None, south: None, west: None, northeast: Some(3), southeast: None, southwest: None, northwest: Some(3) }, Vec2 { x: 3, y: 4 }: RoadCoordNode { coord: Vec2 { x: 3, y: 4 }, north: None, east: None, south: None, west: None, northeast: None, southeast: None, southwest: None, northwest: Some(0) }, Vec2 { x: 5, y: 6 }: RoadCoordNode { coord: Vec2 { x: 5, y: 6 }, north: None, east: None, south: None, west: None, northeast: Some(3), southeast: None, southwest: None, northwest: Some(3) }} }
//     todo!()
// }
