/// Navigation command where distances are specified in absolute game cell
/// counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsoluteCommand {
    Forward(usize),
    Rotate(AbsoluteCommandRotation),
}

/// Navigation command to rotate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbsoluteCommandRotation {
    Left,
    Right,
}
