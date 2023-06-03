use crate::mapp::{Building, MapGrid, Road, RoadOrientation};
use iter_tools::Itertools;
use thiserror::Error;
use vek::Vec2;

pub struct Builder {
    dim: Vec2<usize>,
    roads: Vec<Road>,
    buildings: Vec<Building>,
}

impl Builder {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            dim: Vec2::new(width, height),
            roads: vec![],
            buildings: vec![],
        }
    }

    pub fn road(
        mut self,
        origin: Vec2<usize>,
        length: usize,
        orientation: RoadOrientation,
        rank: u8,
        name: Option<String>,
    ) -> Self {
        let road = Road::new(origin, length, orientation, rank, name);
        self.roads.push(road);
        self
    }

    pub fn building(mut self, origin: Vec2<usize>, dim: Vec2<usize>, name: Option<String>) -> Self {
        let id = self.buildings.len();
        let building = Building::new(id, origin, dim, name);
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
            return Err(BuilderError::UnconnectedBuilding(building));
        }

        grid.buildings.push(building);
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum BuilderError {
    #[error("roads overlap")]
    OverlappingRoads(Road, Road),
    #[error("buildings overlap")]
    OverlappingBuilings(Building, Building),
    #[error("road and building overlap")]
    OverlappingRoadAndBuilding(Road, Building),
    #[error("road is out of map bounds")]
    OutOfBoundsRoad(Road),
    #[error("building is out of map bounds")]
    OutOfBoundsBuilding(Building),
    #[error("building is not connected to a road")]
    UnconnectedBuilding(Building),
}
