use super::{Map, RoadOrientation};
use csv::StringRecord;
use std::io;
use thiserror::Error;
use vek::Vec2;

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
    let mut map = Map::new(Vec2::new(width, height));

    let mut reader = csv::Reader::from_reader(road_csv_reader);
    let mut row_roads = reader
        .records()
        .map(|rslt| {
            let record = rslt.map_err(ReadError::RoadCsv)?;
            parse_road_row(&record).ok_or(ReadError::ParseRoad(record))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // More minor roads (with higher ranks) will be inserted before more major
    // roads. If the two roads intersect at a cell, then the last road to
    // inserted will overwrite the old information at that cell. Therefore, each
    // cell will contain the ID of the more major road that passes through it.
    row_roads.sort_by_key(|row| row.rank);
    row_roads.reverse();
    row_roads
        .into_iter()
        .for_each(|row| map.add_road(row.origin, row.length, row.orientation, row.rank, row.name));

    let mut reader = csv::Reader::from_reader(building_csv_reader);
    reader
        .records()
        .map(|rslt| {
            let record = rslt.map_err(ReadError::BuildingCsv)?;
            parse_building_row(&record).ok_or(ReadError::ParseBuilding(record))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .for_each(|row| map.add_building(row.origin, row.dim, row.name));

    Ok(map)
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

#[derive(Debug, Error)]
pub enum ReadError {
    #[error("failed to read road csv")]
    RoadCsv(csv::Error),
    #[error("failed to read building csv")]
    BuildingCsv(csv::Error),
    #[error("failed to parse road in csv")]
    ParseRoad(StringRecord),
    #[error("failed to parse building in csv")]
    ParseBuilding(StringRecord),
}
