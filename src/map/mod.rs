mod building;
mod display;
mod grid;
mod read;
mod road;

use std::io;

use self::grid::Grid;
use building::{Building, BuildingId};
use read::ReadError;
use road::{Road, RoadId, RoadOrientation};
use vek::Vec2;

#[derive(Debug, Clone)]
pub struct Map {
    grid: Grid,
    roads: Vec<Road>,
    buildings: Vec<Building>,
}

impl Map {
    pub fn new(dim: Vec2<usize>) -> Self {
        Self {
            grid: Grid::new(dim),
            roads: vec![],
            buildings: vec![],
        }
    }

    /// Builds the game map from the given CSVs.
    pub fn from_csvs<R1, R2>(
        width: usize,
        height: usize,
        road_csv_reader: R1,
        building_csv_reader: R2,
    ) -> Result<Map, ReadError>
    where
        R1: io::Read,
        R2: io::Read,
    {
        read::from_csvs(width, height, road_csv_reader, building_csv_reader)
    }

    pub fn add_road(
        &mut self,
        origin: Vec2<usize>,
        length: usize,
        orientation: RoadOrientation,
        rank: u8,
        name: Option<String>,
    ) {
        let id = RoadId::new(self.roads.len());
        let road = Road::new(id, origin, length, orientation, rank, name);
        self.grid.add_road(&road);
        self.roads.push(road);
    }

    pub fn add_building(&mut self, origin: Vec2<usize>, dim: Vec2<usize>, name: Option<String>) {
        let id = BuildingId::new(self.buildings.len());
        let building = Building::new(id, origin, dim, name);
        self.grid.add_building(&building);
        self.buildings.push(building);
    }
}
