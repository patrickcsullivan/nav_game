use std::fmt::Display;

use iter_tools::Itertools;
use thiserror::Error;

use crate::cmd::{transform_cmds, AbsoluteCommand, CtxCommand, CtxCommandDistance};
use crate::lang::{LexError, Lexeme, Sentence, SentenceParseError};
use crate::map::{BuildingId, Map};
use crate::pose::Pose;
use crate::ui::build_arrow_tiles;

/// The state of the game.
pub struct State {
    map: Map,

    word_bank: Vec<Lexeme>,

    sentence: String,

    pose: Pose,

    goal: BuildingId,
}

impl State {
    pub fn new(map: Map, word_bank: Vec<Lexeme>, pose: Pose, goal: BuildingId) -> Self {
        Self {
            map,
            word_bank,
            sentence: "".to_string(),
            pose,
            goal,
        }
    }

    pub fn map(&self) -> &Map {
        &self.map
    }

    pub fn word_bank(&self) -> &Vec<Lexeme> {
        &self.word_bank
    }

    pub fn set_sentence(&mut self, val: String) {
        self.sentence = val;
    }

    pub fn cmds_from_sentence(&self) -> Result<Vec<AbsoluteCommand>, CommandError> {
        let lexemes = Lexeme::parse_line(&self.sentence)?;
        let syntax_tree = Sentence::parse(&lexemes)?;
        let ctx_cmds: Vec<CtxCommand> = syntax_tree.into();
        let abs_cmds =
            transform_cmds(&ctx_cmds, &self.map, &self.pose).map_err(CommandError::CmdTransform)?;
        Ok(abs_cmds)
    }

    pub fn apply_sentence_cmds(&mut self) -> Result<(), CommandError> {
        let cmds = self.cmds_from_sentence()?;
        self.pose = self.pose.apply_cmds(&cmds);
        Ok(())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Current sentence: \"{}\"", self.sentence)?;
        writeln!(f, "Current pose: {}", self.pose)?;

        let cmd_or_err = self.cmds_from_sentence();
        writeln!(f, "Command state: {:?}", cmd_or_err)?;

        let arrow = cmd_or_err
            .ok()
            .map(|cmds| build_arrow_tiles(&self.pose, &cmds));
        writeln!(f, "Arrow: {:?}", arrow)
    }
}

/// An error that occurs trying to generate an `AbsoluteCommand` from a String.
#[derive(Debug, Error)]
pub enum CommandError {
    UnrecognizedWords {
        #[from]
        source: LexError,
    },

    SentenceParse {
        #[from]
        source: SentenceParseError,
    },

    CmdTransform(CtxCommand),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::UnrecognizedWords {
                source: LexError(words),
            } => {
                let words_str = words.iter().map(|w| format!("\"{w}\"")).join(", ");
                write!(
                    f,
                    "You cannot use words that are not in the word bank: {words_str}"
                )
            }
            CommandError::SentenceParse { source: _ } => {
                write!(f, "Could not parse sentence. (TODO: Add detail.)")
            }
            CommandError::CmdTransform(ctx_cmd) => {
                match ctx_cmd {
                    CtxCommand::Forward(dist) => {
                        match dist {
                            CtxCommandDistance::ThisOrNextStreet(Some(dir)) => {
                                write!(f, "There is no intersecting street to the {dir} at or past your current position.")
                            }
                            CtxCommandDistance::ThisOrNextStreet(None) => {
                                write!(f, "There is no intersecting street at or past your current position.")
                            }
                            CtxCommandDistance::NthStreet(1, Some(dir)) => {
                                write!(f, "There is no intersecting street to the {dir} past your current position.")
                            }
                            CtxCommandDistance::NthStreet(n, Some(dir)) => {
                                write!(f, "There are not {n} intersecting streets to the {dir} past your current position.")
                            }
                            CtxCommandDistance::NthStreet(1, None) => {
                                write!(
                                    f,
                                    "There is no intersecting street past your current position."
                                )
                            }
                            CtxCommandDistance::NthStreet(n, None) => {
                                write!(f, "There are not {n} intersecting streets past your current position.")
                            }
                        }
                    }
                    // A roation command should never be the cause of a command
                    // transformation error.
                    CtxCommand::Rotate(_) => write!(f, "You cannot rotate."),
                }
            }
        }
    }
}
