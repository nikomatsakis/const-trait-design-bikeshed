#[const_trait]
pub trait Defaulted {
    fn defaulted() -> Self;
}

impl const Defaulted for String {
    fn defaulted() -> Self {
        String::new()
    }
}