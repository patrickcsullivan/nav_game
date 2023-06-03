use vek::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RoadId(usize);

impl RoadId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

/// Orientation that indicates whether a road runs north to south or east to
/// west.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadOrientation {
    /// Indicates that a road runs north to south.
    NorthSouth,

    /// Indicates that a road runs east to west.
    EastWest,
}

/// A road along which players can travel in the game world.
#[derive(Debug, Clone)]
pub struct Road {
    /// Unique identifier.
    id: RoadId,

    /// Origin of the road.
    origin: Vec2<usize>,

    /// Orientation of the road.
    orientation: RoadOrientation,

    /// Length of the road.
    length: usize,

    /// Name of the road.
    name: Option<String>,

    /// The "rank" of a road is used to distinguish major from minor roads.
    /// Larger roads, such as multi-lane highways, have a smaller rank.
    /// Smaller roads, such as dirt roads, have a larger rank.
    rank: u8,
}

impl Road {
    /// Returns a new road.
    pub fn new(
        id: RoadId,
        origin: Vec2<usize>,
        length: usize,
        orientation: RoadOrientation,
        rank: u8,
        name: Option<String>,
    ) -> Self {
        Self {
            id,
            origin,
            length,
            orientation,
            rank,
            name,
        }
    }

    /// Returns the road ID.
    pub fn id(&self) -> RoadId {
        self.id
    }

    /// Returns the coordinates at which the road starts.
    pub fn origin(&self) -> Vec2<usize> {
        self.origin
    }

    /// Returns the orientation of the road.
    pub fn orientation(&self) -> RoadOrientation {
        self.orientation
    }

    /// Returns the coordinates at which the road ends.
    pub fn terminus(&self) -> Vec2<usize> {
        let delta = match self.orientation {
            RoadOrientation::NorthSouth => Vec2::new(0, self.length - 1),
            RoadOrientation::EastWest => Vec2::new(self.length - 1, 0),
        };
        self.origin + delta
    }
}
