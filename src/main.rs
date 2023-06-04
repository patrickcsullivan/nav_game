mod cmd;
mod direction;
mod lang;
mod map;
mod pose;
mod state;

use direction::{CardinalDirection, TurnDirection};
use iter_tools::Itertools;
use lang::Lexeme;
use map::Map;
use pose::Pose;
use state::State;
use std::io::{self, BufRead};

use crate::map::BuildingId;

fn main() {
    let roads_csv = std::fs::File::open("./example_maps/zonat_25x16_roads.csv").unwrap();
    let buildings_csv = std::fs::File::open("./example_maps/zonat_25x16_buildings.csv").unwrap();
    let map = Map::from_csvs(25, 16, roads_csv, buildings_csv).unwrap();
    println!("{}", &map);

    let init_pose = Pose::new(7, 10, CardinalDirection::South);
    let word_bank = Lexeme::all();
    let goal = BuildingId::new(2); // la hospital
    let mut state = State::new(map, word_bank, init_pose, goal);
    print!("{}", state);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line == *"quit" {
            return;
        } else if line == *"apply" {
            let _ = state.apply_sentence_cmds();
        } else if line == *"words" {
            let mut words = state
                .word_bank()
                .iter()
                .map(|w| w.to_string())
                .collect_vec();
            words.sort();
            println!("{:?}", words)
        } else {
            state.set_sentence(line);
        }

        print!("{}", state);
    }
}
