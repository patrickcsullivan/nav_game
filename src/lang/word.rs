use super::Verb;

/// A word that can be used to build a phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Word {
    Verb(Verb),
    Other(Other),
}

impl Word {
    pub fn verb(&self) -> Option<&Verb> {
        match self {
            Word::Verb(v) => Some(v),
            _ => None,
        }
    }

    pub fn other(&self) -> Option<&Other> {
        match self {
            Word::Other(o) => Some(o),
            _ => None,
        }
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        match self {
            Word::Verb(v) => v.to_string(),
            Word::Other(o) => o.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Other {
    /// "in"
    En,

    /// "to"
    A,

    /// "of"
    De,

    // "to the" (masculine)
    Al,

    /// "the" (femenine)
    La,

    /// "the" (masculine)
    El,

    /// "first"
    Primera,

    /// "second"
    Segunda,

    /// "left"
    Izquierda,

    /// "right"
    Derecha,

    /// "end"
    Final,

    /// "all", "everything"
    Todo,

    /// "straight"
    // TODO: What about "recto"?
    Derecho,

    /// "hand" (as in "left hand")
    Mano,
}

impl ToString for Other {
    fn to_string(&self) -> String {
        match self {
            Other::En => "en",
            Other::A => "a",
            Other::De => "de",
            Other::Al => "al",
            Other::La => "la",
            Other::El => "el",
            Other::Primera => "primera",
            Other::Segunda => "segunda",
            Other::Izquierda => "izquierda",
            Other::Derecha => "derecha",
            Other::Final => "final",
            Other::Todo => "todo",
            Other::Derecho => "derecho",
            Other::Mano => "mano",
        }
        .to_string()
    }
}
