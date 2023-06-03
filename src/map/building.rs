use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BuildingId(usize);

impl BuildingId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// A building.
#[derive(Debug, Clone)]
pub struct Building {
    /// Unique identifier.
    id: BuildingId,

    /// Origin of the building, the coordinates of its southwest most corner.
    origin: Vec2<usize>,

    /// Width and height of the building.
    dim: Vec2<usize>,

    /// Name of the building.
    name: Option<String>,
}

impl Building {
    /// Returns a new building.
    pub fn new(
        id: BuildingId,
        origin: Vec2<usize>,
        dim: Vec2<usize>,
        name: Option<String>,
    ) -> Self {
        Self {
            id,
            origin,
            dim,
            name,
        }
    }

    /// Returns the building ID.
    pub fn id(&self) -> BuildingId {
        self.id
    }

    /// Returns the grid coordinates containing the minimum corner of the
    /// building.
    pub fn min(&self) -> Vec2<usize> {
        self.origin
    }

    /// Returns the grid coordinates containing the maximum corner of the
    /// building.
    pub fn max(&self) -> Vec2<usize> {
        let delta = Vec2::new(self.dim.x - 1, self.dim.y - 1);
        self.origin + delta
    }
}
