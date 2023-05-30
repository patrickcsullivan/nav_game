//! This module defines the grammar of the language used in the game to describe
//! directions.

use crate::lang::Lexeme;
use thiserror::Error;

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

/// Prepositional phrase describing a left or right turn.
#[allow(clippy::enum_variant_names)]
pub enum LeftRightTurnPrepPhrase {
    /// "a la izquierda"
    ALaIzquierda,

    /// "a mano izquierda"
    AManoIzquierda,

    /// "a la derecha"
    ALaDerecha,

    /// "a mano derecha"
    AManoDerecha,
}

/// A noun phrase describing a place onto which one may turn.
pub enum TurnableNounPhrase {
    Street(StreetNounPhrase),
}

/// A noun phrase describing the forward direction.
pub enum ForwardNounPhrase {
    Derecho,
    TodoDerecho,
}

/// A noun phrase describing a distance.
pub enum DistanceNounPhrase {
    /// A specified number of blocks.
    NQuadras(Number),
}

/// A noun phrase describing a specific street.
pub enum StreetNounPhrase {
    LaCalle,
    LaCalleOrd(Ordinality),
}

/// A prepositional phrase describing a distance.
pub enum DistancePrepPhrase {
    Hasta(StreetNounPhrase),
}

/// A number.
pub enum Number {
    Uno,
    Dos,
    Tres,
    Quatro,
}

/// The ordering of an item in a sequence.
pub enum Ordinality {
    Primera,
    Primero,
    Segunda,
    Segundo,
    Tercera,
    Tercero,
    Cuarta,
    Cuarto,
}

impl TryFrom<&[Lexeme]> for Ordinality {
    type Error = ParseOrdinalityError;

    fn try_from(words: &[Lexeme]) -> Result<Self, Self::Error> {
        let word = single(words).ok_or(ParseOrdinalityError())?;
        match word {
            Lexeme::Primera => Ok(Ordinality::Primera),
            Lexeme::Primero => Ok(Ordinality::Primero),
            Lexeme::Segunda => Ok(Ordinality::Segunda),
            Lexeme::Segundo => Ok(Ordinality::Segundo),
            Lexeme::Tercera => Ok(Ordinality::Tercera),
            Lexeme::Tercero => Ok(Ordinality::Tercero),
            Lexeme::Cuarta => Ok(Ordinality::Cuarta),
            Lexeme::Cuarto => Ok(Ordinality::Cuarto),
            _ => Err(ParseOrdinalityError()),
        }
    }
}

#[derive(Debug, Error)]
#[error("The words(s) must be an ordinality.")]
pub struct ParseOrdinalityError();

fn single(lexemes: &[Lexeme]) -> Option<&Lexeme> {
    if lexemes.len() > 1 {
        None
    } else {
        lexemes.first()
    }
}
