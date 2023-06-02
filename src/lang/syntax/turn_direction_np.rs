use super::{DefiniteArticle, TurnDirectionNoun};
use crate::lang::{syntax::parse, Lexeme};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TurnDirectionNounPhrase(pub TurnDirectionNoun);

impl TurnDirectionNounPhrase {
    pub fn try_parse(lexemes: &[Lexeme]) -> Result<(Self, &[Lexeme]), ParseError> {
        let (before, dir, after) = parse::split_at_parsable(lexemes, TurnDirectionNoun::try_parse)
            .ok_or(ParseError::MissingDirection)?;

        let ((), rest) = parse::or(
            before,
            |ls| DefiniteArticle::try_parse_la(ls).map(|(_, rest)| ((), rest)),
            |ls| parse::consume_lexeme(ls, Lexeme::Mano),
        )
        .ok_or(ParseError::MissingLaOrMano)?;

        if !rest.is_empty() {
            Err(ParseError::LexemesBetweenLaOrManoAndDirection)
        } else {
            Ok((TurnDirectionNounPhrase(dir), after))
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(r#"The phrase must include a direction, either "izquierda" or "derecha"."#)]
    MissingDirection,

    #[error(r#"The direction must be immediately preceded by either "la" or "mano."#)]
    MissingLaOrMano,

    #[error(r#"The direction must be immediately preceded by either "la" or "mano."#)]
    LexemesBetweenLaOrManoAndDirection,
}

#[cfg(test)]
mod tests {
    use super::{TurnDirectionNoun, TurnDirectionNounPhrase};
    use crate::lang::Lexeme;

    #[test]
    fn parse() {
        let lexemes = [Lexeme::La, Lexeme::Derecha, Lexeme::De];
        let result = TurnDirectionNounPhrase::try_parse(&lexemes).unwrap();
        let expected = (
            TurnDirectionNounPhrase(TurnDirectionNoun::Derecha),
            &lexemes[2..],
        );
        assert_eq!(result.0, expected.0);
    }
}
