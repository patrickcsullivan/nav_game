/// Prepositional phrase describing a left or right turn.
#[allow(clippy::enum_variant_names)]
pub enum LeftRightTurnPrepPhrase {
    /// "a la izquierda"
    ALaIzquierda,

    /// "a mano izquierda"
    AManoIzquierda,

    /// "a la derecha"
    ALaDerecha,

    /// "a mano derecha"
    AManoDerecha,
}
