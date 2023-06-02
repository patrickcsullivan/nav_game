use thiserror::Error;

use super::{StreetNounPhrase, StreetNounPhraseParseError};
use crate::lang::Lexeme;

/// A noun phrase describing a place onto which one may turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnableNounPhrase {
    Street(StreetNounPhrase),
}

impl TurnableNounPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        match StreetNounPhrase::try_parse(lexemes) {
            Ok((snp, rest)) => Ok((TurnableNounPhrase::Street(snp), rest)),
            Err(e) => Err(ParseError::StreetNounPhrase(e)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{0}")]
    StreetNounPhrase(StreetNounPhraseParseError),
}
