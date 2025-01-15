use std::marker::Destruct;

use crate::{const_defaulted_trait::Defaulted, const_eq_trait::Equals};

pub const fn compare_to_default<T>(value: &T) -> bool
where
    T: ~const Defaulted + ~const Equals + ~const Destruct,
{
    let default_value = T::defaulted();
    Equals::equals(value, &default_value)
}

// for comparison purposes:
pub fn compare_to_default_not_const<T>(value: &T) -> bool
where
    T: Defaulted + Equals
{
    let default_value = T::defaulted();
    Equals::equals(value, &default_value)
}
