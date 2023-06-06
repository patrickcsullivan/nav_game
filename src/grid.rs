use crate::direction::CardinalDirection;
use grid::Grid as InnerGrid;
use iter_tools::Itertools;
use vek::Vec2;

#[derive(Debug, Clone)]
pub struct Grid<T>(InnerGrid<T>);

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(dim: Vec2<usize>) -> Self {
        Grid(InnerGrid::new(dim.y, dim.x))
    }

    pub fn size(&self) -> Vec2<usize> {
        let (rows, cols) = self.0.size();
        Vec2::new(cols, rows)
    }

    pub fn get(&self, idx: Vec2<usize>) -> Option<&T> {
        self.0.get(idx.y, idx.x)
    }

    pub fn get_mut(&mut self, idx: Vec2<usize>) -> Option<&mut T> {
        self.0.get_mut(idx.y, idx.x)
    }

    pub fn get_neighbors(&self, idx: Vec2<usize>) -> Neighbors<&T> {
        self.neighbor_indices(idx).and_then(|idx| self.get(idx))
    }

    pub fn get_neighbor(&self, idx: Vec2<usize>, dir: CardinalDirection) -> Option<&T> {
        let neighbors = self.get_neighbors(idx);
        match dir {
            CardinalDirection::North => neighbors.n,
            CardinalDirection::East => neighbors.e,
            CardinalDirection::South => neighbors.s,
            CardinalDirection::West => neighbors.w,
        }
    }

    pub fn inner_grid(&self) -> &InnerGrid<T> {
        &self.0
    }

    fn neighbor_indices(&self, idx: Vec2<usize>) -> Neighbors<Vec2<usize>> {
        let size = self.size();

        let x = idx.x;
        let x_minus = if x > 0 { Some(x - 1) } else { None };
        let x_plus = if x < size.x - 1 { Some(x + 1) } else { None };
        let x = Some(x); // Wrap x in an Option to make it easeier to work with.

        let y = idx.y;
        let y_minus = if y > 0 { Some(y - 1) } else { None };
        let y_plus = if y < size.y - 1 { Some(y + 1) } else { None };
        let y = Some(y); // Wrap y in an Option to make it easeier to work with.

        Neighbors {
            n: try_vec2(x, y_plus),
            ne: try_vec2(x_plus, y_plus),
            e: try_vec2(x_plus, y),
            se: try_vec2(x_plus, y_minus),
            s: try_vec2(x, y_minus),
            sw: try_vec2(x_minus, y_minus),
            w: try_vec2(x_minus, y),
            nw: try_vec2(x_minus, y_plus),
        }
    }
}

pub struct Neighbors<T> {
    pub n: Option<T>,
    pub ne: Option<T>,
    pub e: Option<T>,
    pub se: Option<T>,
    pub s: Option<T>,
    pub sw: Option<T>,
    pub w: Option<T>,
    pub nw: Option<T>,
}

impl<T> Neighbors<T> {
    pub fn and_then<F, U>(self, f: F) -> Neighbors<U>
    where
        F: Fn(T) -> Option<U>,
    {
        Neighbors {
            n: self.n.and_then(&f),
            ne: self.ne.and_then(&f),
            e: self.e.and_then(&f),
            se: self.se.and_then(&f),
            s: self.s.and_then(&f),
            sw: self.sw.and_then(&f),
            w: self.w.and_then(&f),
            nw: self.nw.and_then(f),
        }
    }

    pub fn into_vec(self) -> Vec<T> {
        vec![
            self.n, self.ne, self.e, self.se, self.s, self.sw, self.w, self.nw,
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}

fn try_vec2<T>(x: Option<T>, y: Option<T>) -> Option<Vec2<T>> {
    let x = x?;
    let y = y?;
    Some(Vec2::new(x, y))
}
