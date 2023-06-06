mod arrow;

pub use arrow::{ArrowSegment, ArrowTile, ArrowTurnDirection};
use iter_tools::Itertools;

use crate::{
    cmd::AbsoluteCommand,
    direction::CardinalDirection,
    grid::Grid,
    map::{BuildingId, Cell, Map},
    pose::Pose,
};
use std::fmt::Display;

use self::arrow::Arrow;

pub struct UiGrid(Grid<UiTile>);

impl UiGrid {
    pub fn new(map: &Map, pose: &Pose, cmds: &[AbsoluteCommand], goal: BuildingId) -> Self {
        let arrow = Arrow::new(pose, cmds);

        let grid = map.grid().map(|idx, cell| match cell {
            Cell::Empty => UiTile::empty(),
            Cell::Road(_) => {
                let arrow_tile = arrow.get(idx);
                let player = if idx == pose.position() {
                    Some(pose.direction())
                } else {
                    None
                };
                UiTile::road(player, arrow_tile)
            }
            Cell::Building(b_id) => UiTile::building(*b_id == goal),
        });
        UiGrid(grid)
    }
}

pub enum UiTile {
    Empty,
    Road {
        player: Option<CardinalDirection>,
        arrow_tile: Option<ArrowTile>,
    },
    Building {
        is_goal: bool,
    },
}

impl UiTile {
    pub fn empty() -> Self {
        UiTile::Empty
    }

    pub fn road(player: Option<CardinalDirection>, arrow_tile: Option<ArrowTile>) -> Self {
        Self::Road { player, arrow_tile }
    }

    pub fn building(is_goal: bool) -> Self {
        Self::Building { is_goal }
    }
}

impl Default for UiTile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Display for UiGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strings = self
            .0
            .inner_grid()
            .iter_rows()
            .map(|tiles_iter| {
                let row = UiTileRow(tiles_iter.collect_vec());
                row.to_string()
            })
            .collect_vec();
        row_strings.reverse();
        writeln!(f, "{}", row_strings.join("\n"))
    }
}

struct UiTileRow<'a>(Vec<&'a UiTile>);

impl<'a> Display for UiTileRow<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiles = &self.0;
        let s = tiles.iter().map(|t| t.to_string()).collect_vec().join("");
        write!(f, "{}", s)
    }
}

impl Display for UiTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout
            .set_color(ColorSpec::new().set_bg(Some(Color::Green)))
            .unwrap();

        match self {
            UiTile::Empty => {
                stdout
                    .set_color(ColorSpec::new().set_bg(Some(Color::Green)))
                    .unwrap();
                write!(f, "   ")
            }
            UiTile::Road { player, arrow_tile } => {
                let mut color = ColorSpec::new();
                color.set_bg(Some(Color::Black));
                if let Some(dir) = player {
                    color.set_fg(Some(Color::Red));
                    stdout.set_color(&color).unwrap();
                    render_direction(dir, f)
                } else if let Some(arrow_tile) = arrow_tile {
                    color.set_fg(Some(Color::Blue));
                    stdout.set_color(&color).unwrap();
                    write!(f, "{}", arrow_tile)
                } else {
                    write!(f, "   ")
                }
            }
            UiTile::Building { is_goal } => {
                let bg = if *is_goal {
                    Color::Yellow
                } else {
                    Color::White
                };
                let mut color = ColorSpec::new();
                color.set_bg(Some(bg));
                stdout.set_color(&color).unwrap();
                write!(f, "   ")
            }
        }
    }
}

impl Display for ArrowTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dir = match self.segment() {
            ArrowSegment::StraightInit(dir) => dir,
            ArrowSegment::StraightMid(dir) => dir,
            ArrowSegment::StraightFinal(dir) => dir,
            ArrowSegment::TurnMid(dir) => dir.final_direction(),
            ArrowSegment::TurnFinal(dir) => dir.final_direction(),
        };
        render_direction(&dir, f)
    }
}

fn render_direction(dir: &CardinalDirection, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match dir {
        CardinalDirection::North => write!(f, " ▲ "),
        CardinalDirection::East => write!(f, " ► "),
        CardinalDirection::South => write!(f, " ▼ "),
        CardinalDirection::West => write!(f, " ◄ "),
    }
}
