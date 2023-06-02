use crate::lang::Lexeme;
use thiserror::Error;

use super::{
    parse, DistanceNounPhrase, DistancePrepPhrase, ForwardNounPhrase, LeftRightTurnPrepPhrase,
    LeftRightTurnPrepPhraseParseError, TurnableNounPhrase, TurnableNounPhraseParseError,
};

/// A sentence which is either a delcaration of where something is or a command
/// instructing how to navigate to a destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sentence {
    /// Examples:
    /// * "Está a la derecha."
    /// * "Está a mano derecha."
    EstáTurnPp(LeftRightTurnPrepPhrase),

    /// Extamples:
    /// * "Está en la segunda calle a la derecha."
    /// * "Está en la calle a la izquierda."
    EstáEnNpPp(TurnableNounPhrase, LeftRightTurnPrepPhrase),

    /// Examples:
    /// * "Gira a la derecha."
    /// * "Gira a mano izquierda."
    GiraPp(LeftRightTurnPrepPhrase),

    /// Examples:
    /// * "Gira la segunda calle a la derecha."
    /// * "Gira la calle a la izquierda."
    GiraNpPp(TurnableNounPhrase, LeftRightTurnPrepPhrase),

    /// Examples:
    /// * "Toma la segunda calle a la derecha."
    /// * "Toma la calle a la izquierda."
    TomaNpPp(TurnableNounPhrase, LeftRightTurnPrepPhrase),

    /// Examples:
    /// * Continúa todo derecho dos quadras.
    /// * Continúa derecho una quadra.
    ContinúaNpNp(ForwardNounPhrase, DistanceNounPhrase),

    /// Examples:
    /// * Continúa todo derecho hasta la primero calle.
    /// * Continúa derecho hasta la segundo calle.
    ContinúaNpPp(ForwardNounPhrase, DistancePrepPhrase),
}

impl Sentence {
    pub fn parse(lexemes: &[Lexeme]) -> Result<Self, ParseError> {
        let (s, rest) = Self::try_parse(lexemes)?;

        if rest.is_empty() {
            Ok(s)
        } else {
            Err(ParseError::LexemesAfterPrepositinalPhrase(rest.to_vec()))
        }
    }

    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (first, rest) = lexemes.split_first().ok_or(ParseError::NoWords)?;
        match first {
            Lexeme::Está => Self::try_parse_está(rest),
            Lexeme::Toma => Self::try_parse_toma(rest),
            Lexeme::Gira => Self::try_parse_gira(rest),
            // Lexeme::Continúa => todo!(),
            _ => Err(ParseError::NonInitialVerb(*first)),
        }
    }

    fn try_parse_está(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        if let Some(((), rest)) = parse::consume_lexeme(lexemes, Lexeme::En) {
            let (np, rest) =
                TurnableNounPhrase::try_parse(rest).map_err(ParseError::EstáEnNpPpFirst)?;
            let (pp, rest) =
                LeftRightTurnPrepPhrase::try_parse(rest).map_err(ParseError::EstáEnNpPpSecond)?;
            Ok((Self::EstáEnNpPp(np, pp), rest))
        } else {
            let (pp, rest) =
                LeftRightTurnPrepPhrase::try_parse(lexemes).map_err(ParseError::EstáTurnPpFirst)?;
            Ok((Self::EstáTurnPp(pp), rest))
        }
    }

    fn try_parse_toma(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (np, rest) =
            TurnableNounPhrase::try_parse(lexemes).map_err(ParseError::TomaNpPpFirst)?;
        let (pp, rest) =
            LeftRightTurnPrepPhrase::try_parse(rest).map_err(ParseError::TomaNpPpSecond)?;
        Ok((Self::TomaNpPp(np, pp), rest))
    }

    fn try_parse_gira(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        if let Ok((np, rest)) = TurnableNounPhrase::try_parse(lexemes) {
            let (pp, rest) =
                LeftRightTurnPrepPhrase::try_parse(rest).map_err(ParseError::GiraNpPpSecond)?;
            Ok((Self::GiraNpPp(np, pp), rest))
        } else {
            let (pp, rest) =
                LeftRightTurnPrepPhrase::try_parse(lexemes).map_err(ParseError::GiraPpFirst)?;
            Ok((Self::GiraPp(pp), rest))
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The sentence must contain words.")]
    NoWords,

    #[error("The sentence must start with a verb.")]
    NonInitialVerb(Lexeme),

    #[error("EstáTurnPpFirst: {0}")]
    EstáTurnPpFirst(LeftRightTurnPrepPhraseParseError),

    #[error("EstáEnNpPpFirst: {0}")]
    EstáEnNpPpFirst(TurnableNounPhraseParseError),

    #[error("EstáEnNpPpSecond: {0}")]
    EstáEnNpPpSecond(LeftRightTurnPrepPhraseParseError),

    #[error("TomaNpPpFirst: {0}")]
    TomaNpPpFirst(TurnableNounPhraseParseError),

    #[error("TomaNpPpSecond: {0}")]
    TomaNpPpSecond(LeftRightTurnPrepPhraseParseError),

    #[error("GiraPpFirst: {0}")]
    GiraPpFirst(LeftRightTurnPrepPhraseParseError),

    #[error("GiraNpPpSecond: {0}")]
    GiraNpPpSecond(LeftRightTurnPrepPhraseParseError),

    #[error("The sentence contains unxpected words after the prepositional phrase.")]
    LexemesAfterPrepositinalPhrase(Vec<Lexeme>),
}
