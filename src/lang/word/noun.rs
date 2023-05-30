#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Noun {
    /// "left"
    Izquierda,

    /// "right"
    Derecha,

    /// "end"
    Final,

    /// "street"
    Calle,

    // "block"
    Quadra,

    // "blocks"
    Quadras,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectionNoun {
    /// "left"
    Izquierda,

    /// "right"
    Derecha,

    /// "end"
    Final,

    /// "street"
    Calle,

    // "block"
    Quadra,

    // "blocks"
    Quadras,
}


impl ToString for Noun {
    fn to_string(&self) -> String {
        match self {
            Noun::Izquierda => "izquierda",
            Noun::Derecha => "derecha",
            Noun::Final => "final",
            Noun::Calle => "calle",
            Noun::Quadra => "quadra",
            Noun::Quadras => "quadras",
        }
        .to_string()
    }
}
