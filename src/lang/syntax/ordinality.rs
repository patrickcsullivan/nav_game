use crate::lang::Lexeme;
use thiserror::Error;

use super::gender::{Gender, HasGender};

/// The ordering of an item in a sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ordinality {
    value: usize,
    gender: Gender,
}

impl Ordinality {
    pub fn new(value: usize, gender: Gender) -> Self {
        Self { value, gender }
    }

    pub fn new_masc(value: usize) -> Self {
        Self::new(value, Gender::Masculine)
    }

    pub fn new_fem(value: usize) -> Self {
        Self::new(value, Gender::Feminine)
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }
}

impl Ordinality {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (l, rest) = lexemes.split_first().ok_or(ParseError::NoLexemes)?;
        let ord = match l {
            Lexeme::Primera => Ok(Self::new_fem(1)),
            Lexeme::Primero => Ok(Self::new_masc(1)),
            Lexeme::Segunda => Ok(Self::new_fem(2)),
            Lexeme::Segundo => Ok(Self::new_masc(2)),
            Lexeme::Tercera => Ok(Self::new_fem(3)),
            Lexeme::Tercero => Ok(Self::new_masc(3)),
            Lexeme::Cuarta => Ok(Self::new_fem(4)),
            Lexeme::Cuarto => Ok(Self::new_masc(4)),
            _ => Err(ParseError::NotOrdinality),
        }?;
        Ok((ord, rest))
    }
}

impl HasGender for Ordinality {
    fn gender(&self) -> Gender {
        self.gender
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("The words(s) must be an ordinality.")]
    NotOrdinality,
    #[error("There are no words.")]
    NoLexemes,
}
