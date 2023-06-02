use super::{parse, StreetNounPhrase, StreetNounPhraseParseError};
use crate::lang::Lexeme;

/// A prepositional phrase describing a distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistancePrepPhrase {
    Hasta(StreetNounPhrase),
}

impl DistancePrepPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let ((), rest) =
            parse::consume_lexeme(lexemes, Lexeme::Hasta).ok_or(ParseError::MissingHasta)?;
        let (np, rest) = StreetNounPhrase::try_parse(rest).map_err(ParseError::StreetNp)?;
        Ok((DistancePrepPhrase::Hasta(np), rest))
    }
}

pub enum ParseError {
    MissingHasta,
    StreetNp(StreetNounPhraseParseError),
}
