use iter_tools::Itertools;

/// A phrase that the player constructs to describe navigation instructions. The
/// phrase may or may not be parsable into an actual navigation command.
#[derive(Debug)]
pub struct Phrase(Vec<Word>);

impl Phrase {
    /// Returns an empty phrase.
    pub fn empty() -> Self {
        Self(vec![])
    }

    /// Push the given word onto the end of the phrase.
    pub fn push(&mut self, word: Word) {
        self.0.push(word)
    }

    /// Removes the last word from the phrase and return it, or returns `None`
    /// if the phrase is empty.
    pub fn pop(&mut self) -> Option<Word> {
        self.0.pop()
    }

    /// Returns an iterator over the words in the phrase.
    pub fn words<'a>(&'a self) -> impl Iterator<Item = &'a Word> {
        self.0.iter()
    }
}

impl ToString for Phrase {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(ToString::to_string)
            .collect_vec()
            .join(" ")
    }
}

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
