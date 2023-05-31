mod lang;
mod map;
mod pose;
// mod _state;

use map::Map;

fn main() {
    let roads_csv = std::fs::File::open("./example_maps/zonat_10x7_roads.csv").unwrap();
    let buildings_csv = std::fs::File::open("./example_maps/zonat_10x7_buildings.csv").unwrap();
    let map = Map::from_csvs(10, 7, roads_csv, buildings_csv).unwrap();
    println!("{:?}", map);
}
