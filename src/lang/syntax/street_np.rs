use super::Ordinality;

/// A noun phrase describing a specific street.
pub enum StreetNounPhrase {
    LaCalle,
    LaCalleOrd(Ordinality),
}
