use thiserror::Error;

use crate::lang::Lexeme;

/// A number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    Uno,
    Dos,
    Tres,
    Quatro,
}

impl TryFrom<&[Lexeme]> for Number {
    type Error = ParseNumberError;

    fn try_from(words: &[Lexeme]) -> Result<Self, Self::Error> {
        if let &[word] = words {
            match word {
                Lexeme::Uno => Ok(Number::Uno),
                Lexeme::Dos => Ok(Number::Dos),
                Lexeme::Tres => Ok(Number::Tres),
                Lexeme::Quatro => Ok(Number::Quatro),
                _ => Err(ParseNumberError()),
            }
        } else {
            Err(ParseNumberError())
        }
    }
}

#[derive(Debug, Error)]
#[error("The words(s) must be a number.")]
pub struct ParseNumberError();
