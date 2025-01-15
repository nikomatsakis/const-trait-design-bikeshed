```rust
pub const trait Defaulted {
    fn defaulted() -> Self;
}

impl const Defaulted for String {
    fn defaulted() -> Self {
        String::new()
    }
}
```