mod lex;
pub mod syntax;

pub use lex::{LexError, Lexeme};
pub use syntax::{Sentence, SentenceParseError};

// String -> Tokens -> Lexems -> Syntax Tree (Sentence) -> [Rel Command] -|
//                                                                   Map  +--> [Abs Command]
//                                                              Position -|
