use crate::lang::Lexeme;
use thiserror::Error;

use super::{parse, TurnDirectionNounPhrase, TurnDirectionNounPhraseParseError};

/// Prepositional phrase describing a left or right turn.
///
/// Examples:
/// * "a la izquierda"
/// * "a mano izquierda"
/// * "a la derecha"
/// * "a mano derecha"
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftRightTurnPrepPhrase(TurnDirectionNounPhrase);

impl LeftRightTurnPrepPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (_, rest) =
            parse::consume_lexeme(lexemes, Lexeme::A).ok_or(ParseError::MissingPreposition)?;
        let (np, rest) = TurnDirectionNounPhrase::try_parse(rest)
            .map_err(ParseError::MissingDirectionNounPhrase)?;
        Ok((LeftRightTurnPrepPhrase(np), rest))
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(r#"The preposition "a" must be in the prepositional phrase."#)]
    MissingPreposition,

    #[error(r#"The preposition must contain a direction noun phrase: {0}"#)]
    MissingDirectionNounPhrase(TurnDirectionNounPhraseParseError),
}
