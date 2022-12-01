use std::ops::{Add, Sub};

/// A trait that describes a value that _behaves_ like a number.
///
/// _Numeric_ indicates, that:
///
/// 1. The type is _number_-like, e.g. is sized, ordered, and trivial to copy.
/// 2. The type can be incremented or decremented logically.
/// 3. Has a reasonable default value for die pips, i.e. the equivalent to `1`.
///
/// Additionally, the numeric must know how to convert _from_ a `usize`.
pub trait Numeric
where
    Self: Add + Sub + Copy + Ord + Sized,
{
    /// The minumum value that can be represented, or the equivakent to `1` for this type.
    const MINIMUM: Self;

    /// What value, when added to an existing value, increases the value by `1` or equivalent.
    const STEPONE: Self;

    /// Create a numeric value that is semantically equivalent to the provided number.
    fn from_usize(number: usize) -> Self;

    /// Create a provided number semantically equivalent to this numeric value.
    fn as_usize(&self) -> usize;
}

macro_rules! numeric {
    ($name:ident) => {
        impl Numeric for $name {
            const MINIMUM: Self = 1;
            const STEPONE: Self = 1;

            fn from_usize(number: usize) -> Self {
                number as Self
            }

            fn as_usize(&self) -> usize {
                *self as usize
            }
        }
    };
}

numeric!(u8);
numeric!(u16);
numeric!(u32);
numeric!(u64);
numeric!(u128);
numeric!(usize);
