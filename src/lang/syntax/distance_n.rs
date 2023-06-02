use super::quantity::{HasQuantity, Quantity};
use crate::lang::Lexeme;

/// A noun phrase describing a distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceNoun {
    /// A block or street.
    Quadra(Quantity, Lexeme),
}

impl DistanceNoun {
    pub fn quadra(lexeme: Lexeme) -> Self {
        Self::Quadra(Quantity::Singular, lexeme)
    }

    pub fn quadras(lexeme: Lexeme) -> Self {
        Self::Quadra(Quantity::Plural, lexeme)
    }

    pub fn lexeme(&self) -> Lexeme {
        match self {
            DistanceNoun::Quadra(_, l) => *l,
        }
    }
}

impl DistanceNoun {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (first, rest) = lexemes.split_first().ok_or(ParseError())?;
        match first {
            Lexeme::Calle => Ok(Self::quadra(*first)),
            Lexeme::Calles => Ok(Self::quadras(*first)),
            Lexeme::Quadra => Ok(Self::quadra(*first)),
            Lexeme::Quadras => Ok(Self::quadras(*first)),
            _ => Err(ParseError()),
        }
        .map(|dn| (dn, rest))
    }
}

impl HasQuantity for DistanceNoun {
    fn quantity(&self) -> Quantity {
        match self {
            DistanceNoun::Quadra(q, _) => *q,
        }
    }
}

pub struct ParseError();
