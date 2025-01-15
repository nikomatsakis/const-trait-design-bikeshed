#[const_trait]
pub trait Equals {
    fn equals(&self, other: &Self) -> bool;
}