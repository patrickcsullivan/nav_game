use crate::lang::Lexeme;
use thiserror::Error;

use super::{
    DistanceNounPhrase, DistancePrepPhrase, ForwardNounPhrase, LeftRightTurnPrepPhrase,
    TurnableNounPhrase,
};

/// A sentence which is either a delcaration of where something is or a command
/// instructing how to navigate to a destination.
pub enum Sentence {
    /// Examples:
    /// * "Está a la derecha."
    /// * "Está a mano derecha."
    EstáTurnPp(LeftRightTurnPrepPhrase),

    /// Extamples:
    /// * "Está en la segunda calle a la derecha."
    /// * "Está en la calle a la izquierda."
    EstaEnNpPp(TurnableNounPhrase, LeftRightTurnPrepPhrase),

    /// Examples:
    /// * "Toma la segunda calle a la derecha."
    /// * "Toma la calle a la izquierda."
    TomaNpPp(TurnableNounPhrase, LeftRightTurnPrepPhrase),

    /// Examples:
    /// * "Gira a la derecha."
    /// * "Gira a mano izquierda."
    GiraPp(LeftRightTurnPrepPhrase),

    /// Examples:
    /// * "Gira la segunda calle a la derecha."
    /// * "Gira la calle a la izquierda."
    GiraNpPp(TurnableNounPhrase, LeftRightTurnPrepPhrase),

    /// "continue" (imperitive)
    ///
    /// Examples:
    /// * Continúa todo derecho dos quadras.
    /// * Continúa derecho una quadra.
    ContinúaNpNp(ForwardNounPhrase, DistanceNounPhrase),

    /// "continue" (imperitive)
    ///
    /// Examples:
    /// * Continúa todo derecho hasta la primero calle.
    /// * Continúa derecho hasta la segundo calle.
    ContinúaNpPp(ForwardNounPhrase, DistancePrepPhrase),
}

impl TryFrom<&[Lexeme]> for Sentence {
    type Error = ParseSentenceError;

    fn try_from(words: &[Lexeme]) -> Result<Self, Self::Error> {
        let (first, rest) = words.split_first().ok_or(ParseSentenceError::NoWords)?;
        match first {
            Lexeme::Está => todo!(),
            Lexeme::Toma => todo!(),
            Lexeme::Gira => todo!(),
            Lexeme::Continúa => todo!(),
            _ => Err(ParseSentenceError::NonInitialVerb(*first)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseSentenceError {
    #[error("The sentence must contain words.")]
    NoWords,

    #[error("The sentence must start with a verb.")]
    NonInitialVerb(Lexeme),

    #[error("The verb must be followed by a prespositional phrase.")]
    NoWordsAfterVerb,
}
