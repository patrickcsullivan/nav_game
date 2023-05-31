use super::StreetNounPhrase;

/// A noun phrase describing a place onto which one may turn.
pub enum TurnableNounPhrase {
    Street(StreetNounPhrase),
}
