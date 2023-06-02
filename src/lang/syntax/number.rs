use std::fmt::Display;

use thiserror::Error;

use crate::lang::Lexeme;

use super::gender::{Gender, HasGender};

/// A number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Number {
    /// The number one.
    ///
    /// Examples:
    /// * "un"
    /// * "uno"
    /// * "una"
    Unx(Unx),
    Mas(usize),
}

impl Number {
    pub fn un() -> Self {
        Self::Unx(Unx::Un)
    }

    pub fn una() -> Self {
        Self::Unx(Unx::Una)
    }

    pub fn uno() -> Self {
        Self::Unx(Unx::Uno)
    }

    pub fn dos() -> Self {
        Self::Mas(2)
    }

    pub fn tres() -> Self {
        Self::Mas(3)
    }

    pub fn quatro() -> Self {
        Self::Mas(4)
    }

    pub fn value(&self) -> usize {
        match self {
            Number::Unx(_) => 1,
            Number::Mas(n) => *n,
        }
    }

    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (first, rest) = lexemes.split_first().ok_or(ParseError())?;
        match first {
            Lexeme::Un => Ok(Self::un()),
            Lexeme::Una => Ok(Self::una()),
            Lexeme::Uno => Ok(Self::uno()),
            Lexeme::Dos => Ok(Self::dos()),
            Lexeme::Tres => Ok(Self::tres()),
            Lexeme::Quatro => Ok(Self::quatro()),
            _ => Err(ParseError()),
        }
        .map(|num| (num, rest))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Unx(unx) => write!(f, "{unx}"),
            Number::Mas(2) => write!(f, "dos"),
            Number::Mas(3) => write!(f, "tres"),
            Number::Mas(4) => write!(f, "quatro"),
            Number::Mas(n) => write!(f, "{n}"),
        }
    }
}

/// Variations of the number one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unx {
    Un,
    Uno,
    Una,
}

impl Display for Unx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Unx::Un => "un",
            Unx::Uno => "uno",
            Unx::Una => "una",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Error)]
#[error("The words(s) must be a number.")]
pub struct ParseError();
