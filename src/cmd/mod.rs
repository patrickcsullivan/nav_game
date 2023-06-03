mod abs;
mod ctx;

pub use abs::AbsoluteCommand;
pub use ctx::CtxCommand;

use self::ctx::CtxCommandDistance;
use crate::{direction::TurnDirection, Map, Pose};

/// Uses the context of the `Map` and the player's `Pose` to transform a series
/// of `CtxCommand`s into `AbsCommand`s.
pub fn transform_cmds(cmds: &[CtxCommand], map: &Map, pose: &Pose) -> Option<Vec<AbsoluteCommand>> {
    let mut curr_pose = *pose;
    let mut abs_cmds = vec![];

    for cmd in cmds {
        let mut next_abs_cmds = transform_cmd(cmd, map, &curr_pose)?;
        curr_pose = curr_pose.apply_cmds(&next_abs_cmds);
        abs_cmds.append(&mut next_abs_cmds);
    }

    Some(abs_cmds)
}

/// Uses the context of the `Map` and the player's `Pose` to transform a
/// `CtxCommand` into an `AbsCommand`.
pub fn transform_cmd(cmd: &CtxCommand, map: &Map, pose: &Pose) -> Option<Vec<AbsoluteCommand>> {
    match cmd {
        CtxCommand::Forward(CtxCommandDistance::ThisOrNextStreet(dir)) => {
            if at_intersection(map, pose, *dir) {
                // The pose is already at the destination intersection, so no
                // forward command is need.
                Some(vec![])
            } else {
                let dist = dist_to_nth_street(map, pose, 1, *dir)?;
                Some(vec![AbsoluteCommand::Forward(dist)])
            }
        }
        CtxCommand::Forward(CtxCommandDistance::NthStreet(n, dir)) => {
            let dist = dist_to_nth_street(map, pose, *n, *dir)?;
            Some(vec![AbsoluteCommand::Forward(dist)])
        }
        CtxCommand::Rotate(dir) => Some(vec![AbsoluteCommand::Rotate(*dir)]),
    }
}

/// Returns `true` if the player's current position is at an intersection with a
/// street in the given `TurnDirection` relative to the player's current
/// orientation.
///
/// If no `TurnDirection` is given, this simply returns `true` if the player is
/// at an interestion with a street from any direction.
fn at_intersection(map: &Map, pose: &Pose, dir: Option<TurnDirection>) -> bool {
    let turn_dirs = match dir {
        Some(dir) => vec![dir],
        None => vec![TurnDirection::Left, TurnDirection::Right],
    };

    turn_dirs
        .iter()
        .map(|d| pose.direction().turn(*d))
        .filter_map(|d| map.get_neighbor(pose.position(), d))
        .any(|c| c.is_road())
}

/// Finds the distance to the `n`th street after the player's current position
/// and in the given `TurnDirection` relative to the player's current
/// orientation.
///
/// If no `TurnDirection` is given, this simply returns the distance to `n`th
/// intersection after the player's current position.
fn dist_to_nth_street(
    map: &Map,
    pose: &Pose,
    n: usize,
    dir: Option<TurnDirection>,
) -> Option<usize> {
    let map_size = map.size();
    let mut curr_pose = *pose;
    let mut streets_count = 0;
    let mut dist = 0;

    let turn_dirs = match dir {
        Some(dir) => vec![dir],
        None => vec![TurnDirection::Left, TurnDirection::Right],
    };

    while streets_count < n {
        dist += 1;
        curr_pose = curr_pose.step_forward(map)?;

        let found_round = turn_dirs
            .iter()
            .map(|d| curr_pose.direction().turn(*d))
            .filter_map(|d| map.get_neighbor(curr_pose.position(), d))
            .any(|c| c.is_road());

        if found_round {
            streets_count += 1;
        }
    }

    Some(dist)
}

pub enum TransformError {}
