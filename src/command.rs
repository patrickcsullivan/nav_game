use iter_tools::Itertools;
use thiserror::Error;

use crate::phrase::{Phrase, Word};

/// A navigation command. The command may or may not be executable depending on
/// the map and the players location when the command is applied.
pub enum Command {
    Forward(ForwardCommand),
    Turn(TurnCommand),
    AndThen(Box<Command>),
}

impl Command {
    pub fn parse(phrase: &Phrase) -> Result<Self, ParseError> {
        let words = phrase.words().collect_vec();

        let verb_indices = words.iter().positions(|w| w.is_verb()).collect_vec();
        if verb_indices.is_empty() {
            return Err(ParseError::NoVerb);
        } else if verb_indices.len() > 1 {
            return Err(ParseError::MultipleVerbs);
        } else if verb_indices[0] != 0 {
            return Err(ParseError::WordsBeforeVerb);
        };

        // The unwrap is ok because we checked that words is not empty.
        let (verb, rest) = words.split_first().unwrap();

        match verb {
            Word::Está => todo!(),
            Word::Toma => todo!(),
            Word::Gira => todo!(),
            Word::Continúa => todo!(),
            Word::En => todo!(),
            Word::A => todo!(),
            Word::De => todo!(),
            Word::Al => todo!(),
            Word::La => todo!(),
            Word::El => todo!(),
            Word::Primera => todo!(),
            Word::Segunda => todo!(),
            Word::Izquierda => todo!(),
            Word::Derecha => todo!(),
            Word::Final => todo!(),
            Word::Todo => todo!(),
            Word::Derecho => todo!(),
            Word::Mano => todo!(),
        }

        todo!()
    }
}

/// An error that can occur while parsing a `Phrase` into a `Command`.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The sentence must contain a verb.")]
    NoVerb,

    #[error("The sentence cannot contain multiple verbs.")]
    MultipleVerbs,

    #[error("The verb should should come first in this phrase.")]
    WordsBeforeVerb,

    #[error("The verb must be followed by a prespositional phrase.")]
    NoWordsAfterVerb,
}

/// Distance to move forward.
pub enum ForwardCommand {
    Blocks(usize),
}

/// Direction to turn.
pub enum TurnCommand {
    Right,
    Left,
    Around,
}
