use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjective {
    Ordinality(Ordinality),
}

impl Display for Adjective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Adjective::Ordinality(o) => write!(f, "{o}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordinality {
    /// "first"
    Primera,

    /// "second"
    Segunda,
}

impl Display for Ordinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Ordinality::Primera => "primera",
            Ordinality::Segunda => "segunda",
        };
        write!(f, "{s}")
    }
}
