use crate::lang::Lexeme;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnDirectionNoun {
    Izquierda,
    Derecha,
}

impl TurnDirectionNoun {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (l, rest) = lexemes.split_first().ok_or(ParseError::Empty)?;
        let article = match l {
            Lexeme::Izquierda => Ok(Self::Izquierda),
            Lexeme::Derecha => Ok(Self::Derecha),
            _ => Err(ParseError::Unrecognized(*l)),
        }?;
        Ok((article, rest))
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("There are no words to parse as a direction.")]
    Empty,

    #[error("\"{0}\" is not a direction. It must be either \"izquierda\" or \"derecha\".")]
    Unrecognized(Lexeme),
}
