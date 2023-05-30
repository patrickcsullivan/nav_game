use super::Word;
use iter_tools::Itertools;

/// A sentence that the player constructs to describe navigation instructions.
/// The sentence may or may not be parsable into an actual navigation command.
#[derive(Debug)]
pub struct Sentence(Vec<Word>);

impl Sentence {
    /// Returns an empty sentence.
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Push the given word onto the end of the sentence.
    pub fn push(&mut self, word: Word) {
        self.0.push(word)
    }

    /// Removes the last word from the sentence and return it, or returns `None`
    /// if the sentence is empty.
    pub fn pop(&mut self) -> Option<Word> {
        self.0.pop()
    }

    /// Returns an iterator over the words in the sentence.
    pub fn words<'a>(&'a self) -> impl Iterator<Item = &'a Word> {
        self.0.iter()
    }
}

impl ToString for Sentence {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    }
}
