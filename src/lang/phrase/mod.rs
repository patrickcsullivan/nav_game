mod noun_phrase;
mod sentence;

/// A prepositional phrase describing a direction.
// to the left
// at the end of the street
pub enum DirectionPrepPhrase {}

// eg:
// two blocks
pub enum DistanceNounPhrase {}

// eg:
// the left
pub enum DirectionNounPhrase {
    LaIzquierda,
    ManoIzquierda,
    LaDerecha,
    ManoDerecha,
}
