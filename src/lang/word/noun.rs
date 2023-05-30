use std::fmt::Display;

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

impl Display for Noun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Noun::Izquierda => "izquierda",
            Noun::Derecha => "derecha",
            Noun::Final => "final",
            Noun::Calle => "calle",
            Noun::Quadra => "quadra",
            Noun::Quadras => "quadras",
        };
        write!(f, "{s}")
    }
}
