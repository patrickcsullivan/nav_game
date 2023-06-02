use super::Number;

/// A noun phrase describing a distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceNounPhrase {
    /// A specified number of blocks.
    NQuadras(Number),
}
