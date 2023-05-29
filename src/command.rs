use crate::lang::{Phrase, Verb, Word};
use iter_tools::Itertools;
use thiserror::Error;

/// A navigation command. The command may or may not be executable depending on
/// the map and the players location when the command is applied.
pub enum Command {
    Forward(ForwardCommand),
    Turn(TurnCommand),
    AndThen(Box<Command>),
}

impl Command {
    pub fn parse(phrase: &Phrase) -> Result<Self, ParseError> {
        let words = phrase.words().map(|a| a.clone()).collect_vec();
        let (verb, rest) = Self::split_verb(&words)?;

        match verb {
            Verb::Está => todo!(),
            Verb::Toma => todo!(),
            Verb::Gira => todo!(),
            Verb::Continúa => todo!(),
        }

        todo!()
    }

    /// Splits the phrase into an inital verb and the words that follow.
    ///
    /// Returns an error if there is not exactly one verb and the phrase or if
    /// the verb is not the first word in the phrase.
    fn split_verb(words: &[Word]) -> Result<(&Verb, &[Word]), ParseError> {
        let (first, rest) = words.split_first().ok_or(ParseError::NoWords)?;
        let verb = first.verb().ok_or(ParseError::NonInitialVerb)?;
        Ok((verb, rest))
    }
}

/// An error that can occur while parsing a `Phrase` into a `Command`.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The sentence must contain words.")]
    NoWords,

    #[error("The sentence must start with a verb.")]
    NonInitialVerb,

    #[error("The verb must be followed by a prespositional phrase.")]
    NoWordsAfterVerb,
}

/// Command to move forward.
pub enum ForwardCommand {
    Blocks(usize),
}

/// Command to turn.
pub enum TurnCommand {
    Right,
    Left,
    Around,
}
