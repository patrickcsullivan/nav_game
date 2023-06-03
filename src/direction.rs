#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    /// Returns the new cardinal direction after turning..
    pub fn turn(&self, dir: TurnDirection) -> Self {
        match dir {
            TurnDirection::Left => self.left(),
            TurnDirection::Right => self.right(),
        }
    }

    /// Returns the new cardinal direction after turning to the right.
    pub fn right(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
            CardinalDirection::West => CardinalDirection::North,
        }
    }

    /// Returns the new cardinal direction after turning to the left.
    pub fn left(&self) -> Self {
        match self {
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
            CardinalDirection::West => CardinalDirection::South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnDirection {
    Left,
    Right,
}
