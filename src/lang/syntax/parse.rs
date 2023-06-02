use crate::lang::Lexeme;

pub fn split_at_parsable<F, T, E>(lexemes: &[Lexeme], parse: F) -> Option<(&[Lexeme], T, &[Lexeme])>
where
    F: Fn(&[Lexeme]) -> Result<(T, &[Lexeme]), E>,
{
    for (i, _) in lexemes.iter().enumerate() {
        let range = &lexemes[i..];
        if let Ok((t, after)) = parse(range) {
            let before = &lexemes[0..i];
            return Some((before, t, after));
        }
    }

    None
}

pub fn or<F1, F2, T>(lexemes: &[Lexeme], parse1: F1, parse2: F2) -> Option<(T, &[Lexeme])>
where
    F1: Fn(&[Lexeme]) -> Option<(T, &[Lexeme])>,
    F2: Fn(&[Lexeme]) -> Option<(T, &[Lexeme])>,
{
    parse1(lexemes).or_else(|| parse2(lexemes))
}

pub fn consume_lexeme(lexemes: &[Lexeme], to_consume: Lexeme) -> Option<((), &[Lexeme])> {
    lexemes
        .split_first()
        .filter(|(&l, _rest)| l == to_consume)
        .map(|(_l, rest)| ((), rest))
}
