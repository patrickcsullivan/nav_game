use crate::mapp::RoadOrientation;

use super::{Builder, BuilderError, MapGrid};
use csv::StringRecord;
use std::io;
use thiserror::Error;
use vek::Vec2;

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("failed to build map")]
    Build(#[from] BuilderError),
    #[error("failed to read road csv")]
    RoadCsv(csv::Error),
    #[error("failed to read building csv")]
    BuildingCsv(csv::Error),
    #[error("failed to parse road in csv")]
    ParseRoad(StringRecord),
    #[error("failed to parse building in csv")]
    ParseBuilding(StringRecord),
}

/// Row from the buildings CSV.
struct BuildingCsvRow {
    origin: Vec2<usize>,
    dim: Vec2<usize>,
    name: Option<String>,
}

impl BuildingCsvRow {
    pub fn new(origin: Vec2<usize>, dim: Vec2<usize>, name: Option<String>) -> Self {
        Self { origin, dim, name }
    }
}

/// Row from the roads CSV.
struct RoadCsvRow {
    origin: Vec2<usize>,
    orientation: RoadOrientation,
    length: usize,
    name: Option<String>,
    rank: u8,
}

impl RoadCsvRow {
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
}

pub fn from_csvs<R1, R2>(
    width: usize,
    height: usize,
    road_csv_reader: R1,
    building_csv_reader: R2,
) -> Result<MapGrid, ReadError>
where
    R1: io::Read,
    R2: io::Read,
{
    let mut builder = Builder::new(width, height);

    let mut reader = csv::Reader::from_reader(road_csv_reader);
    for result in reader.records() {
        let record = result.map_err(ReadError::RoadCsv)?;
        let row = parse_road_row(&record).ok_or(ReadError::ParseRoad(record))?;
        builder = builder.road(row.origin, row.length, row.orientation, row.rank, row.name)
    }

    let mut reader = csv::Reader::from_reader(building_csv_reader);
    for result in reader.records() {
        let record = result.map_err(ReadError::BuildingCsv)?;
        let row = parse_building_row(&record).ok_or(ReadError::ParseBuilding(record))?;
        builder = builder.building(row.origin, row.dim, row.name)
    }

    let grid = builder.build()?;
    Ok(grid)
}

fn parse_road_row(record: &StringRecord) -> Option<RoadCsvRow> {
    let orientation = record.get(0).and_then(parse_orientation)?;
    let origin_x = record.get(1).and_then(|s| str::parse::<usize>(s).ok())?;
    let origin_y = record.get(2).and_then(|s| str::parse::<usize>(s).ok())?;
    let length = record.get(3).and_then(|s| str::parse::<usize>(s).ok())?;
    let rank = record.get(4).and_then(|s| str::parse::<u8>(s).ok())?;
    let name = record.get(5).and_then(non_empty);

    let origin = Vec2::new(origin_x, origin_y);
    let road = RoadCsvRow::new(origin, length, orientation, rank, name);
    Some(road)
}

fn parse_building_row(record: &StringRecord) -> Option<BuildingCsvRow> {
    let origin_x = record.get(0).and_then(|s| str::parse::<usize>(s).ok())?;
    let origin_y = record.get(1).and_then(|s| str::parse::<usize>(s).ok())?;
    let dim_x = record.get(2).and_then(|s| str::parse::<usize>(s).ok())?;
    let dim_y = record.get(3).and_then(|s| str::parse::<usize>(s).ok())?;
    let name = record.get(4).and_then(non_empty);

    let origin = Vec2::new(origin_x, origin_y);
    let dim = Vec2::new(dim_x, dim_y);
    let building = BuildingCsvRow::new(origin, dim, name);
    Some(building)
}

fn parse_orientation(s: &str) -> Option<RoadOrientation> {
    match s {
        "ns" => Some(RoadOrientation::NorthSouth),
        "ew" => Some(RoadOrientation::EastWest),
        _ => None,
    }
}

fn non_empty(s: &str) -> Option<String> {
    match s.trim() {
        "" => None,
        s => Some(s.to_string()),
    }
}
