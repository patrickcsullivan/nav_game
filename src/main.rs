mod lang;
mod map;
mod pose;
// mod _state;

use map::Map;
use std::io::{self, BufRead};

use crate::lang::{Lexeme, Sentence};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == *"quit" {
            return;
        }

        let lexemes_rslt = Lexeme::parse_line(&line);
        println!("{:?}", lexemes_rslt);

        if let Ok(lexemes) = lexemes_rslt {
            let s_rslt = Sentence::parse(&lexemes);
            println!("{:?}", s_rslt);
        }
    }

    // let roads_csv = std::fs::File::open("./example_maps/zonat_10x7_roads.csv").unwrap();
    // let buildings_csv = std::fs::File::open("./example_maps/zonat_10x7_buildings.csv").unwrap();
    // let map = Map::from_csvs(10, 7, roads_csv, buildings_csv).unwrap();
    // println!("{:?}", map);
}
