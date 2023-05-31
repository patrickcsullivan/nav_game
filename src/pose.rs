use thiserror::Error;
use vek::Vec2;

use crate::lang::{AbsoluteCommand, AbsoluteCommandRotation};

/// A position and orientation in the game world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pose {
    position: Vec2<usize>,
    orientation: Orientation,
}

/// Orientation in a cardinal direction in the game world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    /// Returns the new orientation after rotating to the right.
    pub fn right(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }

    /// Returns the new orientation after rotating to the left.
    pub fn left(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        }
    }
}

impl Pose {
    /// Returns a new pose;
    pub fn new(x: usize, y: usize, orientation: Orientation) -> Self {
        Self {
            position: Vec2::new(x, y),
            orientation,
        }
    }

    /// Applies the absolute command to the given pose and returns the resulting
    /// pose.
    pub fn apply_cmd(&self, cmd: &AbsoluteCommand) -> Result<Self, CommandApplicationError> {
        match cmd {
            AbsoluteCommand::Rotate(rotation) => Ok(self.rotate(*rotation)),
            AbsoluteCommand::Forward(distance) => self.move_forward(*distance),
        }
    }

    /// Applies the absolute commands to the given pose and returns the
    /// resulting pose.
    pub fn apply_cmds(&self, cmds: &[AbsoluteCommand]) -> Result<Self, CommandApplicationError> {
        let mut pose = *self;
        for cmd in cmds {
            pose = pose.apply_cmd(cmd)?;
        }
        Ok(pose)
    }

    /// Returns a new pose that has been rotated in the specified direction.
    fn rotate(&self, rotation: AbsoluteCommandRotation) -> Self {
        let new_orientation = match rotation {
            AbsoluteCommandRotation::Left => self.orientation.left(),
            AbsoluteCommandRotation::Right => self.orientation.right(),
        };

        Self {
            position: self.position,
            orientation: new_orientation,
        }
    }

    /// Returns a new pose that has been moved forward the given distance.
    fn move_forward(&self, distance: usize) -> Result<Self, CommandApplicationError> {
        let delta = match self.orientation {
            Orientation::North => Vec2::new(0, distance as isize),
            Orientation::East => Vec2::new(distance as isize, 0),
            Orientation::South => Vec2::new(0, distance as isize * -1),
            Orientation::West => Vec2::new(distance as isize * -1, 0),
        };

        let position: Vec2<isize> = Vec2::new(self.position.x as isize, self.position.y as isize);
        let new_position = position + delta;

        if new_position.iter().all(|&val| val >= 0) {
            let new_position = new_position.map(|val| val as usize);
            Ok(Self {
                position: new_position,
                orientation: self.orientation,
            })
        } else {
            Err(CommandApplicationError::NegativeOutOfBounds(
                *self, distance,
            ))
        }
    }
}

#[derive(Debug, Error)]
pub enum CommandApplicationError {
    #[error("applying the command would move the player into negative coordinates")]
    NegativeOutOfBounds(Pose, usize),
}
