#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quantity {
    Singular,
    Plural,
}

pub trait HasQuantity {
    fn quantity(&self) -> Quantity;

    fn is_sing(&self) -> bool {
        self.quantity() == Quantity::Singular
    }

    fn is_pl(&self) -> bool {
        self.quantity() == Quantity::Plural
    }
}
