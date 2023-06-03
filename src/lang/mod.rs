mod lex;
pub mod syntax;

pub use lex::Lexeme;
pub use syntax::Sentence;

// String -> Tokens -> Lexems -> Syntax Tree (Sentence) -> [Rel Command] -|
//                                                                   Map  +--> [Abs Command]
//                                                              Position -|
