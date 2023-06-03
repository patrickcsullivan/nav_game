use iter_tools::Itertools;

use super::{grid::Cell, Map};
use std::fmt::Display;

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strings = self
            .grid
            .inner_grid()
            .iter_rows()
            .map(|cells_iter| {
                let row = Row(cells_iter.collect_vec());
                row.to_string()
            })
            .collect_vec();
        row_strings.reverse();
        writeln!(f, "{}", row_strings.join("\n"))
    }
}

struct Row<'a>(Vec<&'a Cell>);

impl<'a> Display for Row<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cells = &self.0;
        let s = cells
            .into_iter()
            .map(|c| c.to_string())
            .collect_vec()
            .join("");
        write!(f, "{}", s)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "  "),
            Cell::Road(_) => write!(f, "░░"),
            Cell::Building(_) => write!(f, "██"),
        }
    }
}
