mod lang;
mod map;
mod pose;

use crate::lang::{Lexeme, Sentence};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == *"quit" {
            return;
        }

        if let Ok(lexemes) = Lexeme::parse_line(&line) {
            let s_rslt = Sentence::parse(&lexemes);
            println!("{:?}", s_rslt);
        }
    }

    // let roads_csv = std::fs::File::open("./example_maps/zonat_10x7_roads.csv").unwrap();
    // let buildings_csv = std::fs::File::open("./example_maps/zonat_10x7_buildings.csv").unwrap();
    // let map = Map::from_csvs(10, 7, roads_csv, buildings_csv).unwrap();
    // println!("{:?}", map);
}
