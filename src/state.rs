use crate::lang::{Lexeme, Sentence};
use crate::map::grid::MapGrid;

/// The state of the game.
pub struct State {
    map_grid: MapGrid,

    /// The work-in-progroess phrase that the player is currently building.
    wip_phrase: Sentence,

    completed_phrases: Vec<Sentence>,

    /// A bank of words that the player can select from to build phrases.∑
    word_bank: Vec<Lexeme>,
}

impl State {
    /// Push the word at the given index in the word bank onto the `wip_phrase`.
    pub fn push_word_onto_phrase(&mut self, index: usize) {
        if let Some(word) = self.word_bank.get(index) {
            self.wip_phrase.push(*word);
        }
    }

    /// Remove the last word from the `wip_phrase`.
    pub fn remove_word_from_phrase(&mut self) {
        self.wip_phrase.pop();
    }
}
