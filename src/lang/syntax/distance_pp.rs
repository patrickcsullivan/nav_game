use super::StreetNounPhrase;

/// A prepositional phrase describing a distance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistancePrepPhrase {
    Hasta(StreetNounPhrase),
}
