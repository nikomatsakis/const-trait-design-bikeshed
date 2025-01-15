use crate::const_defaulted_trait::Defaulted;

fn uses_const_bound<T: const Defaulted>() -> T {
    let v = const { T::defaulted() };
    v
}