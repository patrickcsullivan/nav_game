mod builder;
mod read;

pub use builder::{Builder, BuilderError};
pub use read::{from_csvs, ReadError};

use iter_tools::Itertools;
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

/// A road along which players can travel in the game world.
#[derive(Debug, Clone)]
pub struct Road {
    /// Origin of the road.
    origin: Vec2<usize>,

    /// Orientation of the road.
    orientation: RoadOrientation,

    /// Length of the road.
    length: usize,

    /// Name of the road.
    name: Option<String>,

    /// The "rank" of a road is used to distinguish major from minor roads.
    /// Larger roads, such as multi-lane highways, have a smaller rank.
    /// Smaller roads, such as dirt roads, have a larger rank.
    rank: u8,
}

impl Road {
    /// Returns a new road.
    pub fn new(
        origin: Vec2<usize>,
        length: usize,
        orientation: RoadOrientation,
        rank: u8,
        name: Option<String>,
    ) -> Self {
        Self {
            origin,
            length,
            orientation,
            rank,
            name,
        }
    }

    /// Returns the terminus of the road.
    pub fn terminus(&self) -> Vec2<usize> {
        let delta = match self.orientation {
            RoadOrientation::NorthSouth => Vec2::new(0, self.length),
            RoadOrientation::EastWest => Vec2::new(self.length, 0),
        };
        self.origin + delta
    }

    /// Returns `true` if the two roads are colinear and overlap.
    pub fn overlaps_road(&self, other: &Road) -> bool {
        if self.orientation != other.orientation {
            return false;
        }

        match self.orientation {
            RoadOrientation::NorthSouth => {
                if self.origin.x != other.origin.x {
                    false
                } else {
                    is_overlap(
                        self.origin.y,
                        self.terminus().y,
                        other.origin.y,
                        other.terminus().y,
                    )
                }
            }
            RoadOrientation::EastWest => {
                if self.origin.y != other.origin.y {
                    false
                } else {
                    is_overlap(
                        self.origin.x,
                        self.terminus().x,
                        other.origin.x,
                        other.terminus().x,
                    )
                }
            }
        }
    }

    /// Returns `true` if the road and the building overlap.
    pub fn overlaps_building(&self, building: &Building) -> bool {
        match self.orientation {
            RoadOrientation::NorthSouth => {
                is_inside_excl(self.origin.x, building.origin.x, building.max().x)
                    && is_overlap(
                        self.origin.y,
                        self.terminus().y,
                        building.origin.y,
                        building.max().y,
                    )
            }
            RoadOrientation::EastWest => {
                is_inside_excl(self.origin.y, building.origin.y, building.max().y)
                    && is_overlap(
                        self.origin.x,
                        self.terminus().x,
                        building.origin.x,
                        building.max().x,
                    )
            }
        }
    }
}

/// A building.
#[derive(Debug, Clone)]
pub struct Building {
    /// Unique identifier.
    id: usize,

    /// Origin of the building, the coordinates of its southwest most corner.
    origin: Vec2<usize>,

    /// Width and height of the building.
    dim: Vec2<usize>,

    /// Name of the building.
    name: Option<String>,
}

impl Building {
    /// Returns a new building.
    pub fn new(id: usize, origin: Vec2<usize>, dim: Vec2<usize>, name: Option<String>) -> Self {
        Self {
            id,
            origin,
            dim,
            name,
        }
    }

    /// Returns the building ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Returns the maximum coordinates of the building.
    pub fn max(&self) -> Vec2<usize> {
        self.origin + self.dim
    }

    /// Returns `true` if the building and the road overlap.
    pub fn overlaps_road(&self, road: &Road) -> bool {
        match road.orientation {
            RoadOrientation::NorthSouth => {
                is_inside_excl(road.origin.x, self.origin.x, self.max().x)
                    && is_overlap(
                        road.origin.y,
                        road.terminus().y,
                        self.origin.y,
                        self.max().y,
                    )
            }
            RoadOrientation::EastWest => {
                is_inside_excl(road.origin.y, self.origin.y, self.max().y)
                    && is_overlap(
                        road.origin.x,
                        road.terminus().x,
                        self.origin.x,
                        self.max().x,
                    )
            }
        }
    }

    /// Returns `true` if the buildings overlap.
    pub fn overlaps_building(&self, other: &Building) -> bool {
        is_overlap(self.origin.x, self.max().x, other.origin.x, other.max().x)
            && is_overlap(self.origin.y, self.max().y, other.origin.y, other.max().y)
    }

    /// Returns the coordinates where there is a connection between the road and
    /// the buiding.
    ///
    /// Each returned coordinate is also associated with a
    /// `CoordBuildingAdjacency` that describes the position of the building
    /// relative to the coordinate.
    pub fn get_connections(&self, road: &Road) -> Vec<(Vec2<usize>, CoordBuildingAdjacency)> {
        match road.orientation {
            RoadOrientation::NorthSouth => {
                let x = if self.origin.x == road.origin.x {
                    Some(self.origin.x)
                } else if self.max().x == road.origin.x {
                    Some(self.max().x)
                } else {
                    None
                };

                if let Some(x) = x {
                    let is_building_east_of_road = x == self.origin.x;
                    let (default_adjacency, southmost_adjacency, northmost_adjacency) =
                        if is_building_east_of_road {
                            (
                                CoordBuildingAdjacency::NortheastSouthEast,
                                CoordBuildingAdjacency::Northeast,
                                CoordBuildingAdjacency::Southeast,
                            )
                        } else {
                            (
                                CoordBuildingAdjacency::SouthwestNorthwest,
                                CoordBuildingAdjacency::Northwest,
                                CoordBuildingAdjacency::Southwest,
                            )
                        };

                    let coords = intersection_incl(
                        self.origin.y,
                        self.max().y,
                        road.origin.y,
                        road.terminus().y,
                    )
                    .collect_vec();

                    coords
                        .iter()
                        .enumerate()
                        .map(|(i, &y)| {
                            let adjacency = if i == 0 {
                                southmost_adjacency
                            } else if i == coords.len() - 1 {
                                northmost_adjacency
                            } else {
                                default_adjacency
                            };
                            (Vec2::new(x, y), adjacency)
                        })
                        .collect_vec()
                } else {
                    vec![]
                }
            }
            RoadOrientation::EastWest => {
                let y = if self.origin.y == road.origin.y {
                    Some(self.origin.y)
                } else if self.max().y == road.origin.y {
                    Some(self.max().y)
                } else {
                    None
                };

                if let Some(y) = y {
                    let is_building_north_of_road = y == self.origin.y;
                    let (default_adjacency, westmost_adjacency, eastmost_adjacency) =
                        if is_building_north_of_road {
                            (
                                CoordBuildingAdjacency::NorthwestNortheast,
                                CoordBuildingAdjacency::Northeast,
                                CoordBuildingAdjacency::Northwest,
                            )
                        } else {
                            (
                                CoordBuildingAdjacency::SoutheastSouthWest,
                                CoordBuildingAdjacency::Southeast,
                                CoordBuildingAdjacency::Southwest,
                            )
                        };

                    let coords = intersection_incl(
                        self.origin.x,
                        self.max().x,
                        road.origin.x,
                        road.terminus().x,
                    )
                    .collect_vec();

                    coords
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| {
                            let adjacency = if i == 0 {
                                westmost_adjacency
                            } else if i == coords.len() - 1 {
                                eastmost_adjacency
                            } else {
                                default_adjacency
                            };
                            (Vec2::new(x, y), adjacency)
                        })
                        .collect_vec()
                } else {
                    vec![]
                }
            }
        }
    }
}

/// Orientation that indicates whether a road runs north to south or east to
/// west.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadOrientation {
    /// Indicates that a road runs north to south.
    NorthSouth,

    /// Indicates that a road runs east to west.
    EastWest,
}

/// Represents the different types of adjacencies that a single building can
/// have to a coordinate on a road.
pub enum CoordBuildingAdjacency {
    Northeast,
    Southeast,
    Southwest,
    Northwest,
    NorthwestNortheast,
    NortheastSouthEast,
    SoutheastSouthWest,
    SouthwestNorthwest,
}

/// Returns `true` if there is overlap between two ranges.
///
/// If the two ranges are adjacent (i.e., `first_min == second_max` or
/// `second_min == first_max`) then they are not considered overlapping and this
/// returns `false`.
#[inline]
fn is_overlap(first_min: usize, first_max: usize, second_min: usize, second_max: usize) -> bool {
    !(first_max <= second_min || first_min >= second_max)
}

/// Given two inclusive integer ranges, returns an iterator over the integers in
/// the intersection of the two ranges.
#[inline]
fn intersection_incl(
    first_min: usize,
    first_max: usize,
    second_min: usize,
    second_max: usize,
) -> impl Iterator<Item = usize> {
    first_min.max(second_min)..=first_max.min(second_max)
}

/// Returns `true` if `val` is inside the exclusive range.
#[inline]
fn is_inside_excl(val: usize, lower: usize, upper: usize) -> bool {
    val > lower && val < upper
}
