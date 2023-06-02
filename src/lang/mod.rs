mod abs_command;
mod lex;
mod real_command;
pub mod syntax;

pub use abs_command::{AbsoluteCommand, AbsoluteCommandRotation};
pub use lex::Lexeme;
pub use syntax::Sentence;

// String -> Tokens -> Lexems -> Syntax Tree (Sentence) -> [Rel Command] -|
//                                                                   Map  +--> [Abs Command]
//                                                              Position -|
