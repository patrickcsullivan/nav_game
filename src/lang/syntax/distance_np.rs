use thiserror::Error;

use crate::lang::{
    syntax::{number::Unx, quantity::HasQuantity},
    Lexeme,
};

use super::{distance_n::DistanceNoun, parse, Number};

/// A noun phrase describing a distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceNounPhrase {
    /// A specified number of blocks or streets.
    NQuadras(usize),
}

impl DistanceNounPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (before, distance_n, after) =
            parse::split_at_parsable(lexemes, DistanceNoun::try_parse)
                .ok_or(ParseError::MissingDistanceNoun)?;

        let (number, after_number) = Number::try_parse(before)
            .map_err(|_| ParseError::NotImmediatelyPrecededNumberOrIndefiniteArticle)?;

        if !after_number.is_empty() {
            return Err(ParseError::NotImmediatelyPrecededNumberOrIndefiniteArticle);
        }

        // TODO: Change this to match on (number, distance_n.gender,
        // distance_n.quantity) if additional variables of DistanceNoun are
        // added.
        match number {
            Number::Unx(Unx::Un) => {
                if distance_n.is_sing() {
                    Err(ParseError::GenderDisagreement(number, distance_n.lexeme()))
                } else {
                    Err(ParseError::GenderAndQuantityDisagreement(
                        number,
                        distance_n.lexeme(),
                    ))
                }
            }
            Number::Unx(Unx::Una) => {
                if distance_n.is_sing() {
                    Ok((DistanceNounPhrase::NQuadras(1), after))
                } else {
                    Err(ParseError::QuantityDisagreement(
                        number,
                        distance_n.lexeme(),
                    ))
                }
            }
            Number::Unx(Unx::Uno) => {
                if distance_n.is_sing() {
                    Err(ParseError::GenderDisagreement(number, distance_n.lexeme()))
                } else {
                    Err(ParseError::GenderAndQuantityDisagreement(
                        number,
                        distance_n.lexeme(),
                    ))
                }
            }
            Number::Mas(val) => {
                if distance_n.is_pl() {
                    Ok((DistanceNounPhrase::NQuadras(val), after))
                } else {
                    Err(ParseError::QuantityDisagreement(
                        number,
                        distance_n.lexeme(),
                    ))
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(
        "The distance noun must be immediately preceded by a number or an indefinite article."
    )]
    NotImmediatelyPrecededNumberOrIndefiniteArticle,

    #[error(r#"The noun "{1}" and "{0}" must agree in gender."#)]
    GenderDisagreement(Number, Lexeme),

    #[error(r#"The noun "{1}" and "{0}" must agree in number."#)]
    QuantityDisagreement(Number, Lexeme),

    #[error(r#"The noun "{1}" and "{0}" must agree in gender and number."#)]
    GenderAndQuantityDisagreement(Number, Lexeme),

    #[error("The phrase must contain a distance noun.")]
    MissingDistanceNoun,
}
