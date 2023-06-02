use super::{
    gender::{Gender, HasGender},
    quantity::{HasQuantity, Quantity},
};
use crate::lang::Lexeme;

pub struct Article {
    gender: Gender,
    quantity: Quantity,
}

impl Article {
    pub fn fem_sing() -> Self {
        Self {
            gender: Gender::Feminine,
            quantity: Quantity::Singular,
        }
    }

    pub fn try_parse(lexemes: &[Lexeme]) -> Option<(Self, &[Lexeme])> {
        let (l, rest) = lexemes.split_first()?;
        let article = match l {
            Lexeme::La => Some(Self::fem_sing()),
            _ => None,
        }?;
        Some((article, rest))
    }

    pub fn try_parse_la(lexemes: &[Lexeme]) -> Option<(Self, &[Lexeme])> {
        Self::try_parse(lexemes).filter(|(a, rest)| a.is_fem() && a.is_sing())
    }
}

impl HasGender for Article {
    fn gender(&self) -> Gender {
        self.gender
    }
}

impl HasQuantity for Article {
    fn quantity(&self) -> Quantity {
        self.quantity
    }
}
