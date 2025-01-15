```rust
use std::marker::Destruct;

use crate::{const_defaulted_trait::Defaulted, const_eq_trait::Equals};

pub const fn compare_to_default<T>(value: &T) -> bool
where
    T: Defaulted + Equals,
{
    let default_value = T::defaulted();
    Equals::equals(value, &default_value)
}
```