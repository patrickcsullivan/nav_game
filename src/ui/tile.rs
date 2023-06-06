use crate::direction::CardinalDirection;

use super::ArrowTile;

pub enum UiTile {
    Empty,
    Road {
        player: Option<CardinalDirection>,
        arrow: Option<ArrowTile>,
    },
    Building {
        is_goal: bool,
    },
}
