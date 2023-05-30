#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Adjective {
    Ordinality(Ordinality),
}

impl ToString for Adjective {
    fn to_string(&self) -> String {
        match self {
            Adjective::Ordinality(o) => o.to_string(),
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

impl ToString for Ordinality {
    fn to_string(&self) -> String {
        match self {
            Ordinality::Primera => "primera",
            Ordinality::Segunda => "segunda",
        }
        .to_string()
    }
}
