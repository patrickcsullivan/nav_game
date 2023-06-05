use vek::Vec2;

use crate::{
    cmd::AbsoluteCommand,
    direction::{CardinalDirection, TurnDirection},
    pose::Pose,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArrowTile {
    segment: ArrowSegment,
    position: Vec2<usize>,
}

impl ArrowTile {
    pub fn new(segment: ArrowSegment, position: Vec2<usize>) -> Self {
        Self { segment, position }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowSegment {
    StraightInit(CardinalDirection),
    StraightMid(CardinalDirection),
    StraightFinal(CardinalDirection),
    TurnMid(ArrowTurnDirection),
    TurnFinal(ArrowTurnDirection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowTurnDirection {
    NorthToEast,
    NorthToWest,
    EastToNorth,
    EastToSouth,
    SouthToEast,
    SouthToWest,
    WestToNorth,
    WestToSouth,
}

pub fn build_tiles(pose: &Pose, cmds: &[AbsoluteCommand]) -> Vec<ArrowTile> {
    let mut tiles = vec![];
    let mut curr_pose = *pose;

    if let Some((first_cmd, rest_cmds)) = cmds.split_first() {
        // Generate tiles for the first command.
        let mut first_tiles =
            tiles_from_command(&curr_pose, *first_cmd, true, rest_cmds.is_empty());
        append_tiles(&mut tiles, &mut first_tiles);
        curr_pose = curr_pose.apply_cmd(first_cmd);

        if let Some((last_cmd, mid_cmds)) = rest_cmds.split_last() {
            // Generate tiles for the remaining commands except for the last command.
            for cmd in mid_cmds {
                let mut mid_tiles = tiles_from_command(&curr_pose, *cmd, false, false);
                append_tiles(&mut tiles, &mut mid_tiles);
                curr_pose = curr_pose.apply_cmd(cmd);
            }

            // Generate tiles for the last command if there was more than one
            // command.
            let mut last_tiles = tiles_from_command(&curr_pose, *last_cmd, false, true);
            append_tiles(&mut tiles, &mut last_tiles);
        }
    }

    tiles
}

fn append_tiles(tiles: &mut Vec<ArrowTile>, other: &mut Vec<ArrowTile>) {
    let needs_overwrite = tiles
        .last()
        .and_then(|last| other.first().map(|first| (first, last)))
        .filter(|(first, last)| first.position == last.position)
        .is_some();

    if needs_overwrite {
        tiles.pop();
    }

    tiles.append(other);
}

fn tiles_from_command(
    pose: &Pose,
    cmd: AbsoluteCommand,
    is_init: bool,
    is_final: bool,
) -> Vec<ArrowTile> {
    let mut tiles = vec![];

    match cmd {
        AbsoluteCommand::Forward(dist) => {
            if is_init {
                tiles.push(ArrowTile::new(
                    ArrowSegment::StraightInit(pose.direction()),
                    pose.position(),
                ));
            }

            for delta in 1..dist {
                let pos = pose.move_forward_unsafe(delta).position();
                tiles.push(ArrowTile::new(
                    ArrowSegment::StraightMid(pose.direction()),
                    pos,
                ));
            }

            let pos = pose.move_forward_unsafe(dist).position();
            let seg = if is_final {
                ArrowSegment::StraightFinal(pose.direction())
            } else {
                ArrowSegment::StraightMid(pose.direction())
            };
            tiles.push(ArrowTile::new(seg, pos));
        }
        AbsoluteCommand::Rotate(turn_dir) => {
            let arrow_dir = match (pose.direction(), turn_dir) {
                (CardinalDirection::North, TurnDirection::Left) => ArrowTurnDirection::NorthToWest,
                (CardinalDirection::North, TurnDirection::Right) => ArrowTurnDirection::NorthToEast,
                (CardinalDirection::East, TurnDirection::Left) => ArrowTurnDirection::EastToNorth,
                (CardinalDirection::East, TurnDirection::Right) => ArrowTurnDirection::EastToSouth,
                (CardinalDirection::South, TurnDirection::Left) => ArrowTurnDirection::SouthToEast,
                (CardinalDirection::South, TurnDirection::Right) => ArrowTurnDirection::SouthToWest,
                (CardinalDirection::West, TurnDirection::Left) => ArrowTurnDirection::WestToNorth,
                (CardinalDirection::West, TurnDirection::Right) => ArrowTurnDirection::WestToSouth,
            };
            let seg = if is_final {
                ArrowSegment::TurnFinal(arrow_dir)
            } else {
                ArrowSegment::TurnMid(arrow_dir)
            };
            tiles.push(ArrowTile::new(seg, pose.position()));
        }
    }

    tiles
}
