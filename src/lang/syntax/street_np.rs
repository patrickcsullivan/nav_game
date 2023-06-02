use thiserror::Error;

use super::{Article, Ordinality, OrdinalityParseError};
use crate::lang::Lexeme;

/// A noun phrase describing a specific street.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreetNounPhrase {
    LaCalle,
    LaCalleOrd(Ordinality),
}

impl StreetNounPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let idx = lexemes
            .into_iter()
            .position(|&l| l == Lexeme::Calle)
            .ok_or(ParseError::MissingCalle)?;
        let (before_calle, after_incl_calle) = lexemes.split_at(idx);

        // Try to get the article from the start of the lexemes.
        let (_, before_calle) =
            Article::try_parse_la(before_calle).ok_or(ParseError::MissingArticle)?;

        // It is safe to unwrap because we know after_incl_calle includes at
        // least the element at lexemes[idx].
        let (_, after_calle) = after_incl_calle.split_first().unwrap();

        if before_calle.is_empty() {
            // If there are no lexemes between "la" and "calle", then check if
            // "calle" is followed by an ordinality.
            if let Ok((ord, after_ord)) = Ordinality::try_parse(after_calle) {
                Ok((Self::LaCalleOrd(ord), after_ord))
            } else {
                Ok((Self::LaCalle, after_calle))
            }
        } else {
            // If there are lexemes between "la" and "calle" then they must be
            // an ordinality.
            let (ord, after_ord) = Ordinality::try_parse(before_calle)
                .map_err(ParseError::LexemesBeforeCalleNotOrdinality)?;
            Ok((Self::LaCalleOrd(ord), after_calle))
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The noun phrase must contain \"calle\".")]
    MissingCalle,

    #[error("\"calle\" must be preceded by the article \"la\".")]
    MissingArticle,

    #[error("The word between \"la\" and \"calle\" must be an ordinality.")]
    LexemesBeforeCalleNotOrdinality(OrdinalityParseError),
}
