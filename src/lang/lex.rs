//! This module defines different lexemes and the logic for reading lexemes from
//! text.

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

    /// "one"
    Uno,

    /// "two"
    Dos,

    /// "three"
    Tres,

    /// "four"
    Quatro,
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
            Lexeme::Uno => "uno",
            Lexeme::Dos => "dos",
            Lexeme::Tres => "tres",
            Lexeme::Quatro => "quatro",
        };
        write!(f, "{s}")
    }
}

impl FromStr for Lexeme {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "en" => Ok(Lexeme::En),
            "a" => Ok(Lexeme::A),
            "de" => Ok(Lexeme::De),
            "al" => Ok(Lexeme::Al),
            "la" => Ok(Lexeme::La),
            "el" => Ok(Lexeme::El),
            "izquierda" => Ok(Lexeme::Izquierda),
            "derecha" => Ok(Lexeme::Derecha),
            "final" => Ok(Lexeme::Final),
            "todo" => Ok(Lexeme::Todo),
            "derecho" => Ok(Lexeme::Derecho),
            "mano" => Ok(Lexeme::Mano),
            "primera" => Ok(Lexeme::Primera),
            "segunda" => Ok(Lexeme::Segunda),
            "quadra" => Ok(Lexeme::Quadra),
            "quadras" => Ok(Lexeme::Quadras),
            "está" => Ok(Lexeme::Está),
            "toma" => Ok(Lexeme::Toma),
            "gira" => Ok(Lexeme::Gira),
            "continúa" => Ok(Lexeme::Continúa),
            "primero" => Ok(Lexeme::Primero),
            "segundo" => Ok(Lexeme::Segundo),
            "tercera" => Ok(Lexeme::Tercera),
            "tercero" => Ok(Lexeme::Tercero),
            "cuarta" => Ok(Lexeme::Cuarta),
            "cuarto" => Ok(Lexeme::Cuarto),
            "uno" => Ok(Lexeme::Uno),
            "dos" => Ok(Lexeme::Dos),
            "tres" => Ok(Lexeme::Tres),
            "quatro" => Ok(Lexeme::Quatro),
            _ => Err(LexError(s.to_string())),
        }
    }
}

#[derive(Debug, Error)]
#[error("The word \"{0}\" is not recognized.")]
pub struct LexError(String);
