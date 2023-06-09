mod cmd;
mod direction;
mod grid;
mod lang;
mod map;
mod pose;
mod state;
mod ui;

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
    let init_pose = Pose::new(7, 10, CardinalDirection::South);
    let word_bank = Lexeme::all();
    let goal = BuildingId::new(2); // la hospital
    let mut state = State::new(map, word_bank, init_pose, goal);

    state.render_map();
    print!("{}", state);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if line == *"para" {
            return;
        } else if line == *"el mapa" {
            state.render_map();
        } else if line == *"ve" {
            let _ = state.apply_sentence_cmds();
        } else if line == *"las palabras" {
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
