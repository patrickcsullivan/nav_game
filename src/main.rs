mod lang;
mod map;
mod state;

use map::grid::from_csvs;

fn main() {
    let roads_csv = std::fs::File::open("./example_maps/zonat_10x7_roads.csv").unwrap();
    let buildings_csv = std::fs::File::open("./example_maps/zonat_10x7_buildings.csv").unwrap();
    let grid = from_csvs(10, 7, roads_csv, buildings_csv).unwrap();
    println!("{:?}", grid);
}
