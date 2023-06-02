use crate::lang::Lexeme;

/// A noun phrase describing the forward direction.
///
/// Examples:
/// * "derecho"
/// * "todo derecho"
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ForwardNounPhrase();

impl ForwardNounPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (first, rest) = lexemes.split_first().ok_or(ParseError())?;
        match *first {
            Lexeme::Derecho => Ok((ForwardNounPhrase(), rest)),
            Lexeme::Todo => {
                let (second, rest) = rest.split_first().ok_or(ParseError())?;
                match *second {
                    Lexeme::Derecho => Ok((ForwardNounPhrase(), rest)),
                    _ => Err(ParseError()),
                }
            }
            _ => Err(ParseError()),
        }
    }
}

pub struct ParseError();
