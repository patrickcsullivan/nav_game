use super::grid::{Building, CoordBuildingAdjacency, MapGrid};
use iter_tools::Itertools;
use vek::Vec2;

/// A node in the graph that represents some building.
pub struct BuildingNode {
    building: Building,
}

impl BuildingNode {
    pub fn new(building: Building) -> Self {
        Self { building }
    }
}

/// A node in the graph that represents a coordinate along some road.
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
pub struct MapGraph {
    building_nodes: Vec<BuildingNode>,
    road_coord_nodes: Vec<RoadCoordNode>,
}

impl MapGraph {
    /// Constructs and returns a graph from the roads and buildings in the given
    /// `MapGrid`.
    pub fn new(grid: &MapGrid) -> Self {
        let mut graph = Self {
            building_nodes: vec![],
            road_coord_nodes: vec![],
        };

        let building_nodes = grid
            .buildings()
            .into_iter()
            .map(|b| BuildingNode::new(b.clone()))
            .collect_vec();
        graph.building_nodes = building_nodes;

        for r in grid.roads() {
            for b in building_nodes.iter().map(|node| node.building) {
                for (coord, adj) in b.get_connections(r) {
                    let mut coord_node = graph.get_or_create_road_coord(coord);
                    coord_node.set_building_adjacencies(b.id(), adj);
                }
            }
        }

        todo!()
    }

    /// Returns the building node with the given ID.
    pub fn get_building(&self, id: usize) -> Option<&BuildingNode> {
        // It happens to the be case that a building's ID corresponds to its
        // index in the list of buildings so we can simply access it by index.
        self.building_nodes.get(id)
    }

    /// Returns the road coordinate node with the given coordinates.
    pub fn get_road_cooord(&self, coord: Vec2<usize>) -> Option<&RoadCoordNode> {
        self.road_coord_nodes
            .iter()
            .find(|node| node.coord == coord)
    }

    /// Gets the road coordinate node with the given coordinates or creates and
    /// adds a new node with the coordinates to the graph if one does not exist
    /// yet.
    fn get_or_create_road_coord(&mut self, coord: Vec2<usize>) -> &RoadCoordNode {
        if let Some(node) = self.get_road_cooord(coord) {
            node
        } else {
            let node = RoadCoordNode::new(coord);
            self.road_coord_nodes.push(node);

            // We can safely unwrap because there is guaranteed to be at least
            // one element since we just pushed the new node.
            self.road_coord_nodes.last().unwrap()
        }
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
