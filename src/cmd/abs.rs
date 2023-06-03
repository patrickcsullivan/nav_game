use crate::{direction::TurnDirection, map::Map};

/// Navigation command where distances are specified in absolute game cell
/// counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsoluteCommand {
    Forward(usize),
    Rotate(TurnDirection),
}
