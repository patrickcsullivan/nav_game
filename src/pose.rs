use thiserror::Error;
use vek::Vec2;

use crate::{
    direction::{CardinalDirection, TurnDirection},
    lang::AbsoluteCommand,
};

/// A position and direction in the game world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pose {
    position: Vec2<usize>,
    direction: CardinalDirection,
}

impl Pose {
    /// Returns a new pose;
    pub fn new(x: usize, y: usize, direction: CardinalDirection) -> Self {
        Self {
            position: Vec2::new(x, y),
            direction,
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
    fn rotate(&self, dir: TurnDirection) -> Self {
        let new_direction = match dir {
            TurnDirection::Left => self.direction.left(),
            TurnDirection::Right => self.direction.right(),
        };

        Self {
            position: self.position,
            direction: new_direction,
        }
    }

    /// Returns a new pose that has been moved forward the given distance.
    fn move_forward(&self, distance: usize) -> Result<Self, CommandApplicationError> {
        let delta = match self.direction {
            CardinalDirection::North => Vec2::new(0, distance as isize),
            CardinalDirection::East => Vec2::new(distance as isize, 0),
            CardinalDirection::South => Vec2::new(0, distance as isize * -1),
            CardinalDirection::West => Vec2::new(distance as isize * -1, 0),
        };

        let position: Vec2<isize> = Vec2::new(self.position.x as isize, self.position.y as isize);
        let new_position = position + delta;

        if new_position.iter().all(|&val| val >= 0) {
            let new_position = new_position.map(|val| val as usize);
            Ok(Self {
                position: new_position,
                direction: self.direction,
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
