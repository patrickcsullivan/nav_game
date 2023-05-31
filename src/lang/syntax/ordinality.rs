use crate::lang::Lexeme;
use thiserror::Error;

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
        if let &[word] = words {
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
        } else {
            Err(ParseOrdinalityError())
        }
    }
}

#[derive(Debug, Error)]
#[error("The words(s) must be an ordinality.")]
pub struct ParseOrdinalityError();
