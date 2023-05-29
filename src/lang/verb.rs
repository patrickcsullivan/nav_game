#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verb {
    /// "it is"
    Está,

    /// "you take" (imperitive)
    Toma,

    /// "you turn" (imperitive)
    Gira,

    /// "continue" (imperitive)
    Continúa,
}

impl ToString for Verb {
    fn to_string(&self) -> String {
        match self {
            Verb::Está => "está",
            Verb::Toma => "toma",
            Verb::Gira => "gira",
            Verb::Continúa => "continúa",
        }
        .to_string()
    }
}
