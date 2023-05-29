/// A word that can be used to build a phrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Word {
    /// "it is"
    Está,

    /// "you take" (imperitive)
    Toma,

    /// "you turn" (imperitive)
    Gira,

    /// "continue" (imperitive)
    Continúa,

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

impl Word {
    pub fn is_verb(&self) -> bool {
        match self {
            Word::Está => true,
            Word::Toma => true,
            Word::Gira => true,
            Word::Continúa => true,
            _ => false,
        }
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        match self {
            Word::Está => "está",
            Word::Toma => "toma",
            Word::Gira => "gira",
            Word::Continúa => "continúa",
            Word::En => "en",
            Word::A => "a",
            Word::De => "de",
            Word::Al => "al",
            Word::La => "la",
            Word::El => "el",
            Word::Primera => "primera",
            Word::Segunda => "segunda",
            Word::Izquierda => "izquierda",
            Word::Derecha => "derecha",
            Word::Final => "final",
            Word::Todo => "todo",
            Word::Derecho => "derecho",
            Word::Mano => "mano",
        }
        .to_string()
    }
}
