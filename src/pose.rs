use crate::{
    cmd::AbsoluteCommand,
    direction::{CardinalDirection, TurnDirection},
    Map,
};
use std::fmt::Display;
use vek::Vec2;

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

    /// Returns the position of the player.
    pub fn position(&self) -> Vec2<usize> {
        self.position
    }

    /// Returns the orientation of the player.
    pub fn direction(&self) -> CardinalDirection {
        self.direction
    }

    /// Applies the absolute command to the given pose and returns the resulting
    /// pose.
    pub fn apply_cmd(&self, cmd: &AbsoluteCommand) -> Self {
        match cmd {
            AbsoluteCommand::Rotate(rotation) => self.rotate(*rotation),
            AbsoluteCommand::Forward(distance) => self.move_forward_unsafe(*distance),
        }
    }

    /// Applies the absolute commands to the given pose and returns the
    /// resulting pose.
    pub fn apply_cmds(&self, cmds: &[AbsoluteCommand]) -> Self {
        let mut pose = *self;
        for cmd in cmds {
            pose = pose.apply_cmd(cmd);
        }
        pose
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

    /// Returns a new pose that has been moved forward one cell in its current
    /// orientation along a street.
    pub fn step_forward(&self, map: &Map) -> Option<Self> {
        let delta = match self.direction {
            CardinalDirection::North => Vec2::new(0, 1),
            CardinalDirection::East => Vec2::new(1, 0),
            CardinalDirection::South => Vec2::new(0, -1),
            CardinalDirection::West => Vec2::new(-1, 0),
        };

        let position: Vec2<isize> = Vec2::new(self.position.x as isize, self.position.y as isize);
        let new_position = position + delta;

        // This would step out of the lower bounds.
        if new_position.iter().any(|&val| val < 0) {
            return None;
        }

        let new_position = new_position.map(|val| val as usize);
        if let Some(cell) = map.get(new_position) {
            if cell.is_road() {
                Some(Pose {
                    position: new_position,
                    direction: self.direction,
                })
            } else {
                None
            }
        } else {
            // This would step out of the upper bounds.
            None
        }
    }

    /// Returns a new pose that has been moved forward the given distance.
    pub fn move_forward_unsafe(&self, distance: usize) -> Self {
        let delta = match self.direction {
            CardinalDirection::North => Vec2::new(0, distance as isize),
            CardinalDirection::East => Vec2::new(distance as isize, 0),
            CardinalDirection::South => Vec2::new(0, -(distance as isize)),
            CardinalDirection::West => Vec2::new(-(distance as isize), 0),
        };

        let position: Vec2<isize> = Vec2::new(self.position.x as isize, self.position.y as isize);
        let new_position = position + delta;
        let new_position = new_position.map(|val| val as usize);

        Self {
            position: new_position,
            direction: self.direction,
        }
    }
}

impl Display for Pose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}), {}",
            self.position.x, self.position.y, self.direction
        )
    }
}
