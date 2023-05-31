/// Navigation command where distances are specified in real world terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealWorldCommand {
    Forward(RealWorldCommandDistance),
    Rotate(RealWorldCommandRotation),
}

/// Navigation command to move forward.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealWorldCommandDistance {
    Blocks(usize),
}

/// Navigation command to rotate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RealWorldCommandRotation {
    Right,
    Left,
}
