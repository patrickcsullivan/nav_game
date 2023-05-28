mod examples;

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

/// A road along which players can travel in the game world.
#[derive(Debug, Clone)]
pub struct Road {
    /// Origin of the road.
    origin: Vec2<usize>,

    /// Length of the road.
    length: usize,

    /// Orientation of the road.
    orientation: Orientation,

    /// Name of the road.
    name: Option<String>,

    /// The "rank" of a road is used to distinguish major from minor roads.
    /// Larger roads, such as multi-lane highways, have a smaller rank.
    /// Smaller roads, such as dirt roads, have a larger rank.
    rank: u8,
}

impl Road {
    /// Returns a new named road.
    pub fn new(
        origin: Vec2<usize>,
        length: usize,
        orientation: Orientation,
        rank: u8,
        name: String,
    ) -> Self {
        Self {
            origin,
            length,
            orientation,
            rank,
            name: Some(name),
        }
    }

    /// Returns a new unnamed road.
    pub fn new_unnamed(
        origin: Vec2<usize>,
        length: usize,
        orientation: Orientation,
        rank: u8,
    ) -> Self {
        Self {
            origin,
            length,
            orientation,
            rank,
            name: None,
        }
    }

    /// Returns the terminus of the road.
    pub fn terminus(&self) -> Vec2<usize> {
        let delta = match self.orientation {
            Orientation::NorthSouth => Vec2::new(0, self.length),
            Orientation::EastWest => Vec2::new(self.length, 0),
        };
        self.origin + delta
    }

    /// Returns `true` if the two roads are colinear and overlap.
    pub fn overlaps_road(&self, other: &Road) -> bool {
        if self.orientation != other.orientation {
            return false;
        }

        let (self_cross_pos, self_min, self_max, other_cross_pos, other_min, other_max) =
            match self.orientation {
                Orientation::NorthSouth => (
                    self.origin.x,
                    self.origin.y,
                    self.terminus().y,
                    other.origin.x,
                    other.origin.y,
                    other.terminus().y,
                ),
                Orientation::EastWest => (
                    self.origin.y,
                    self.origin.x,
                    self.terminus().x,
                    other.origin.x,
                    other.origin.y,
                    other.terminus().y,
                ),
            };

        // Roads are colinear...
        self_cross_pos == other_cross_pos
            // ...and they overlap.
            && is_overlap(self_min, self_max, other_min, other_max)
    }

    /// Returns `true` if the road and the building overlap.
    pub fn overlaps_building(&self, building: &Building) -> bool {
        match self.orientation {
            Orientation::NorthSouth => {
                is_inside_excl(self.origin.x, building.origin.x, building.max().x)
                    && is_overlap(
                        self.origin.y,
                        self.terminus().y,
                        building.origin.y,
                        building.max().y,
                    )
            }
            Orientation::EastWest => {
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
    /// Origin of the building.
    origin: Vec2<usize>,

    /// Width and height of the building.
    dim: Vec2<usize>,

    /// Name of the building.
    name: Option<String>,
}

impl Building {
    /// Returns a new named building.
    pub fn new(origin: Vec2<usize>, dim: Vec2<usize>, name: String) -> Self {
        Self {
            origin,
            dim,
            name: Some(name),
        }
    }

    /// Returns a new unnamed building.
    pub fn new_unnamed(origin: Vec2<usize>, dim: Vec2<usize>) -> Self {
        Self {
            origin,
            dim,
            name: None,
        }
    }

    /// Returns the maximum coordinates of the building.
    pub fn max(&self) -> Vec2<usize> {
        self.origin + self.dim
    }

    /// Returns `true` if the building and the road overlap.
    pub fn overlaps_road(&self, road: &Road) -> bool {
        match road.orientation {
            Orientation::NorthSouth => {
                is_inside_excl(road.origin.x, self.origin.x, self.max().x)
                    && is_overlap(
                        road.origin.y,
                        road.terminus().y,
                        self.origin.y,
                        self.max().y,
                    )
            }
            Orientation::EastWest => {
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
    pub fn get_connections(&self, road: &Road) -> Vec<Vec2<usize>> {
        match road.orientation {
            Orientation::NorthSouth => {
                let x = if self.origin.x == road.origin.x {
                    Some(self.origin.x)
                } else if self.max().x == road.origin.x {
                    Some(self.max().x)
                } else {
                    None
                };

                if let Some(x) = x {
                    intersection_incl(
                        self.origin.y,
                        self.max().y,
                        road.origin.y,
                        road.terminus().y,
                    )
                    .map(|y| Vec2::new(x, y))
                    .collect_vec()
                } else {
                    vec![]
                }
            }
            Orientation::EastWest => {
                let y = if self.origin.y == road.origin.y {
                    Some(self.origin.y)
                } else if self.max().y == road.origin.y {
                    Some(self.max().y)
                } else {
                    None
                };

                if let Some(y) = y {
                    intersection_incl(
                        self.origin.x,
                        self.max().x,
                        road.origin.x,
                        road.terminus().x,
                    )
                    .map(|x| Vec2::new(x, y))
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
pub enum Orientation {
    /// Indicates that a road runs north to south.
    NorthSouth,

    /// Indicates that a road runs east to west.
    EastWest,
}

pub struct MapGridBuilder {
    dim: Vec2<usize>,
    roads: Vec<Road>,
    buildings: Vec<Building>,
}

impl MapGridBuilder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            dim: Vec2::new(width, height),
            roads: vec![],
            buildings: vec![],
        }
    }

    pub fn road(mut self, road: Road) -> Self {
        self.roads.push(road);
        self
    }

    pub fn building(mut self, building: Building) -> Self {
        self.buildings.push(building);
        self
    }

    pub fn build(self) -> Result<MapGrid, BuilderError> {
        let mut grid = MapGrid {
            dim: self.dim,
            roads: vec![],
            buildings: vec![],
        };

        for r in self.roads {
            Self::try_append_road(&mut grid, r)?
        }

        for b in self.buildings {
            Self::try_append_building(&mut grid, b)?
        }

        Ok(grid)
    }

    /// Tries to add the given road to the map.
    ///
    /// Returns an error if the road overlaps another road or if the road
    /// intersects a building.
    fn try_append_road(grid: &mut MapGrid, road: Road) -> Result<(), BuilderError> {
        if road.terminus().x > grid.dim.x || road.terminus().y > grid.dim.y {
            return Err(BuilderError::OutOfBoundsRoad(road));
        }

        for r in &grid.roads {
            if road.overlaps_road(r) {
                return Err(BuilderError::OverlappingRoads(r.clone(), road));
            }
        }

        for b in &grid.buildings {
            if road.overlaps_building(b) {
                return Err(BuilderError::OverlappingRoadAndBuilding(road, b.clone()));
            }
        }

        grid.roads.push(road);
        Ok(())
    }

    /// Tries to add the given building to the map.
    ///
    /// Returns an error if the building intersects a road, if the building
    /// intersects annother building, or if the building is not connected to any
    /// road.
    fn try_append_building(grid: &mut MapGrid, building: Building) -> Result<(), BuilderError> {
        if building.max().x > grid.dim.x || building.max().y > grid.dim.y {
            return Err(BuilderError::OutOfBoundsBuilding(building));
        }

        for r in &grid.roads {
            if building.overlaps_road(r) {
                return Err(BuilderError::OverlappingRoadAndBuilding(
                    r.clone(),
                    building,
                ));
            }
        }

        for b in &grid.buildings {
            if building.overlaps_building(b) {
                return Err(BuilderError::OverlappingBuilings(b.clone(), building));
            }
        }

        let is_disconnected = grid
            .roads
            .iter()
            .flat_map(|r| building.get_connections(r))
            .collect_vec()
            .is_empty();
        if is_disconnected {
            return Err(BuilderError::UnreachableBuilding(building));
        }

        grid.buildings.push(building);
        Ok(())
    }
}

pub enum BuilderError {
    OverlappingRoads(Road, Road),
    OverlappingBuilings(Building, Building),
    OverlappingRoadAndBuilding(Road, Building),
    OutOfBoundsRoad(Road),
    OutOfBoundsBuilding(Building),
    UnreachableBuilding(Building),
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
