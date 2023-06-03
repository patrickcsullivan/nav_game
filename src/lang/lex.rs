//! This module defines different lexemes and the logic for reading lexemes from
//! text.

use iter_tools::Itertools;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// A lexeme or word that can be used to build a sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lexeme {
    /// "first" (feminine)
    Primera,

    /// "first" (masculine)
    Primero,

    /// "second" (feminine)
    Segunda,

    /// "second" (masculine)
    Segundo,

    /// "second" (feminine)
    Tercera,

    /// "second" (masculine)
    Tercero,

    /// "second" (feminine)
    Cuarta,

    /// "second" (masculine)
    Cuarto,

    /// "in"
    En,

    /// "to"
    A,

    /// "of"
    De,

    // "to the" (masculine)
    Al,

    /// "the" (femenine)
    La,

    /// "the" (masculine)
    El,

    /// "left"
    Izquierda,

    /// "right"
    Derecha,

    /// "end"
    Final,

    // "street"
    Calle,

    // "streets"
    Calles,

    // "block"
    Quadra,

    // "blocks"
    Quadras,

    /// "all", "everything"
    Todo,

    /// "straight"
    // TODO: What about "recto"?
    Derecho,

    /// "hand" (as in "left hand")
    Mano,

    /// "it is"
    Está,

    /// "you take" (imperitive)
    Toma,

    /// "you turn" (imperitive)
    Gira,

    /// "continue" (imperitive)
    Continúa,

    // "a" or "one (masculine)"
    Un,

    // "one (feminine)"
    Una,

    /// "one (masculine)"
    Uno,

    /// "two"
    Dos,

    /// "three"
    Tres,

    /// "four"
    Quatro,

    // "until"
    Hasta,
}

impl Lexeme {
    pub fn parse_line(line: &str) -> Result<Vec<Lexeme>, LexError> {
        let (lexemes, unknowns): (Vec<_>, Vec<_>) = line
            .split_whitespace()
            .map(|s| Lexeme::from_str(s).map_err(|FromStrError(s)| s))
            .partition_result();

        if unknowns.is_empty() {
            Ok(lexemes)
        } else {
            Err(LexError(unknowns))
        }
    }

    fn from_lowercase(s: &str) -> Option<Lexeme> {
        match s {
            "en" => Some(Lexeme::En),
            "a" => Some(Lexeme::A),
            "de" => Some(Lexeme::De),
            "al" => Some(Lexeme::Al),
            "la" => Some(Lexeme::La),
            "el" => Some(Lexeme::El),
            "izquierda" => Some(Lexeme::Izquierda),
            "derecha" => Some(Lexeme::Derecha),
            "final" => Some(Lexeme::Final),
            "todo" => Some(Lexeme::Todo),
            "derecho" => Some(Lexeme::Derecho),
            "mano" => Some(Lexeme::Mano),
            "primera" => Some(Lexeme::Primera),
            "segunda" => Some(Lexeme::Segunda),
            "quadra" => Some(Lexeme::Quadra),
            "quadras" => Some(Lexeme::Quadras),
            "está" => Some(Lexeme::Está),
            "toma" => Some(Lexeme::Toma),
            "gira" => Some(Lexeme::Gira),
            "continúa" => Some(Lexeme::Continúa),
            "primero" => Some(Lexeme::Primero),
            "segundo" => Some(Lexeme::Segundo),
            "tercera" => Some(Lexeme::Tercera),
            "tercero" => Some(Lexeme::Tercero),
            "cuarta" => Some(Lexeme::Cuarta),
            "cuarto" => Some(Lexeme::Cuarto),
            "un" => Some(Lexeme::Un),
            "una" => Some(Lexeme::Una),
            "uno" => Some(Lexeme::Uno),
            "dos" => Some(Lexeme::Dos),
            "tres" => Some(Lexeme::Tres),
            "quatro" => Some(Lexeme::Quatro),
            "calle" => Some(Lexeme::Calle),
            "calles" => Some(Lexeme::Calles),
            "hasta" => Some(Lexeme::Hasta),
            _ => None,
        }
    }

    /// Returns a `Vec` containing all variants of `Lexeme`.
    pub fn all() -> Vec<Lexeme> {
        vec![
            Self::En,
            Self::A,
            Self::De,
            Self::Al,
            Self::La,
            Self::El,
            Self::Izquierda,
            Self::Derecha,
            Self::Final,
            Self::Todo,
            Self::Derecho,
            Self::Mano,
            Self::Primera,
            Self::Segunda,
            Self::Quadra,
            Self::Quadras,
            Self::Está,
            Self::Toma,
            Self::Gira,
            Self::Continúa,
            Self::Primero,
            Self::Segundo,
            Self::Tercera,
            Self::Tercero,
            Self::Cuarta,
            Self::Cuarto,
            Self::Un,
            Self::Una,
            Self::Uno,
            Self::Dos,
            Self::Tres,
            Self::Quatro,
            Self::Calle,
            Self::Calles,
            Self::Hasta,
        ]
    }
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Lexeme::En => "en",
            Lexeme::A => "a",
            Lexeme::De => "de",
            Lexeme::Al => "al",
            Lexeme::La => "la",
            Lexeme::El => "el",
            Lexeme::Izquierda => "izquierda",
            Lexeme::Derecha => "derecha",
            Lexeme::Final => "final",
            Lexeme::Todo => "todo",
            Lexeme::Derecho => "derecho",
            Lexeme::Mano => "mano",
            Lexeme::Primera => "primera",
            Lexeme::Segunda => "segunda",
            Lexeme::Quadra => "quadra",
            Lexeme::Quadras => "quadras",
            Lexeme::Está => "está",
            Lexeme::Toma => "toma",
            Lexeme::Gira => "gira",
            Lexeme::Continúa => "continúa",
            Lexeme::Primero => "primero",
            Lexeme::Segundo => "segundo",
            Lexeme::Tercera => "tercera",
            Lexeme::Tercero => "tercero",
            Lexeme::Cuarta => "cuarta",
            Lexeme::Cuarto => "cuarto",
            Lexeme::Un => "un",
            Lexeme::Una => "una",
            Lexeme::Uno => "uno",
            Lexeme::Dos => "dos",
            Lexeme::Tres => "tres",
            Lexeme::Quatro => "quatro",
            Lexeme::Calle => "calle",
            Lexeme::Calles => "calles",
            Lexeme::Hasta => "hasta",
        };
        write!(f, "{s}")
    }
}

impl FromStr for Lexeme {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_lowercase(&s.to_lowercase()).ok_or(FromStrError(s.to_string()))
    }
}

#[derive(Debug, Error)]
#[error(r#""{0}" is not a recognized lexeme"#)]
pub struct FromStrError(pub String);

#[derive(Debug, Error)]
#[error("The words are are not recognized.")]
pub struct LexError(pub Vec<String>);
