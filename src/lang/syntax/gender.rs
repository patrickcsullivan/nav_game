#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Feminine,
    Masculine,
    // Neutral,
}

pub trait HasGender {
    fn gender(&self) -> Gender;

    fn is_fem(&self) -> bool {
        self.gender() == Gender::Feminine
    }

    fn is_masc(&self) -> bool {
        self.gender() == Gender::Masculine
    }

    // fn is_neutral(&self) -> bool {
    //     self.gender() == Gender::Neutral
    // }
}
