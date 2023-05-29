use crate::word::Word;
use iter_tools::Itertools;

/// A phrase that the player constructs to describe navigation instructions. The
/// phrase may or may not be parsable into an actual navigation command.
#[derive(Debug)]
pub struct Phrase(Vec<Word>);

impl Phrase {
    /// Returns an empty phrase.
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Push the given word onto the end of the phrase.
    pub fn push(&mut self, word: Word) {
        self.0.push(word)
    }

    /// Removes the last word from the phrase and return it, or returns `None`
    /// if the phrase is empty.
    pub fn pop(&mut self) -> Option<Word> {
        self.0.pop()
    }

    /// Returns an iterator over the words in the phrase.
    pub fn words<'a>(&'a self) -> impl Iterator<Item = &'a Word> {
        self.0.iter()
    }
}

impl ToString for Phrase {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    }
}
