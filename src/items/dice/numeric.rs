use std::{
    fmt::Debug,
    ops::{Add, Sub},
};

use crate::traits::{Numeric, Polyhedral, Rotate, RotateMut, Step, StepMut};

/// A die that starts at `1` and has a defined maximum numeric value.
///
/// A numeric dice is the simplest form of die, and at runtime is represented by a single number.
///
/// # Examples
///
/// ```
/// # use tomb::items::NumericDie;
/// NumericDie::<u8, 4>::new();
/// ```
///
/// Whenever possible, it is recommended to use the provided type aliases for popular formats:
///
/// ```
/// # use tomb::items::D6;
/// D6::new();
/// ```
///
/// # Trivia
///
/// Despite the name, _Numeric_Die does accept non-numbers, as long as they are number-_like_:
///
/// 1. Implement the [`Numeric`] trait.
/// 2. The _default_ value should be `1` or `1`-like.
/// 3. Solemnly swear to behave like numbers so that future traits can utilize them like one.
#[derive(Clone, PartialEq, Eq)]
pub struct NumericDie<T, const MAXIMUM: usize>(T)
where
    T: Numeric;

/// A conveniently provided 4-sided numeric die.
///
/// # Examples
///
/// ```
/// # use tomb::items::D4;
/// # use tomb::traits::Rotate;
/// let die = D4::new().rotate(3);
/// assert_eq!(die.value(), 4);
/// ```
pub type D4 = NumericDie<u8, 4>;

/// A conveniently provided 6-sided numeric die.
///
/// # Examples
///
/// ```
/// # use tomb::items::D6;
/// # use tomb::traits::Rotate;
/// let die = D6::new().rotate(5);
/// assert_eq!(die.value(), 6);
/// ```
pub type D6 = NumericDie<u8, 6>;

/// A conveniently provided 8-sided numeric die.
///
/// # Examples
///
/// ```
/// # use tomb::items::D8;
/// # use tomb::traits::Rotate;
/// let die = D8::new().rotate(7);
/// assert_eq!(die.value(), 8);
/// ```
pub type D8 = NumericDie<u8, 8>;

/// A conveniently provided 10-sided numeric die.
///
/// # Examples
///
/// ```
/// # use tomb::items::D10;
/// # use tomb::traits::Rotate;
/// let die = D10::new().rotate(9);
/// assert_eq!(die.value(), 10);
/// ```
pub type D10 = NumericDie<u8, 10>;

/// A conveniently provided 12-sided numeric die.
///
/// # Examples
///
/// ```
/// # use tomb::items::D12;
/// # use tomb::traits::Rotate;
/// let die = D12::new().rotate(11);
/// assert_eq!(die.value(), 12);
/// ```
pub type D12 = NumericDie<u8, 12>;

/// A conveniently provided 20-sided numeric die.
///
/// # Examples
///
/// ```
/// # use tomb::items::D20;
/// # use tomb::traits::Rotate;
/// let die = D20::new().rotate(19);
/// assert_eq!(die.value(), 20);
/// ```
pub type D20 = NumericDie<u8, 20>;

impl<T, const MAXIMUM: usize> NumericDie<T, MAXIMUM>
where
    T: Numeric,
{
    /// Creates a new die starting at `1` or the equivalent of `1` for non-numbers.
    pub fn new() -> Self {
        Self(T::MINIMUM)
    }

    /// Creates a new die starting at the given `value`.
    ///
    /// # Safety
    ///
    /// The value is _not_ checked for bounds correctness, and could cause undefined behavior.
    unsafe fn from_unchecked(value: T) -> Self {
        Self(value)
    }

    /// Returns the total possible sides for the die.
    pub const fn sides() -> usize {
        MAXIMUM
    }

    /// Returns the currently faced value.
    pub const fn value(&self) -> T {
        self.0
    }
}

impl<T, const MAXIMUM: usize> Debug for NumericDie<T, MAXIMUM>
where
    T: Debug + Numeric,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "D{}:{:?}", MAXIMUM, self.0)?;
        Ok(())
    }
}

impl<T, const MAXIMUM: usize> Default for NumericDie<T, MAXIMUM>
where
    T: Numeric,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const MAXIMUM: usize> From<T> for NumericDie<T, MAXIMUM>
where
    T: Numeric,
{
    /// Converts from the provided die value into a die presenting that value (side).
    ///
    /// # Panics
    ///
    /// If the number is out of range for the capacity of the die.
    fn from(number: T) -> Self {
        assert!(number >= T::MINIMUM);
        assert!(number.as_usize() <= MAXIMUM);
        unsafe { Self::from_unchecked(number) }
    }
}

impl<T, const MAXIMUM: usize> Polyhedral for NumericDie<T, MAXIMUM>
where
    T: Numeric,
{
    fn sides() -> usize {
        Self::sides()
    }
}

impl<T, const MAXIMUM: usize> Step for NumericDie<T, MAXIMUM>
where
    T: Numeric + Add<Output = T> + Sub<Output = T>,
{
    /// Rotates the die forward by 1.
    ///
    /// If the value would have surpassed the maximum, it returns back to the minimum value.
    fn next(&self) -> Self {
        let mut next = self.0 + T::STEPONE;
        if next >= T::from_usize(MAXIMUM) {
            next = T::MINIMUM;
        }
        unsafe { Self::from_unchecked(next) }
    }

    /// Rotates the die backwards by 1.
    ///
    /// If the value would have surpassed the minumum, it returns back to the maximum value.
    fn back(&self) -> Self {
        let mut back = self.0 - T::STEPONE;
        if back < T::MINIMUM {
            back = T::from_usize(MAXIMUM);
        }
        unsafe { Self::from_unchecked(back) }
    }
}

impl<T, const MAXIMUM: usize> StepMut for NumericDie<T, MAXIMUM>
where
    T: Numeric + Add<Output = T> + Sub<Output = T>,
{
    /// Rotates the die forward by 1.
    ///
    /// If the value would have surpassed the maximum, it returns back to the minimum value.
    fn next_mut(&mut self) {
        let mut next = self.0 + T::STEPONE;
        if next >= T::from_usize(MAXIMUM) {
            next = T::MINIMUM;
        }
        self.0 = next;
    }

    /// Rotates the die backwards by 1.
    ///
    /// If the value would have surpassed the minumum, it returns back to the maximum value.
    fn back_mut(&mut self) {
        let mut back = self.0 - T::STEPONE;
        if back < T::MINIMUM {
            back = T::from_usize(MAXIMUM);
        }
        self.0 = back;
    }
}

fn rotate_forward_usize<T, const MAXIMUM: usize>(amount: usize, mut next: usize) -> T
where
    T: Numeric + Add<Output = T> + Sub<Output = T>,
{
    debug_assert!(amount > 0);
    next += amount;
    if next > MAXIMUM {
        next %= MAXIMUM;
    }
    T::from_usize(next)
}

fn rotate_backward_usize<T, const MAXIMUM: usize>(amount: usize, mut next: usize) -> T
where
    T: Numeric + Add<Output = T> + Sub<Output = T>,
{
    debug_assert!(amount > 0);
    println!("next:{next} - amount:{amount}");
    let rotated = next as i64 - (amount as i64);
    if rotated < 1 {
        let rotated = rotated % MAXIMUM as i64 + MAXIMUM as i64;
        next = rotated as usize;
    } else {
        next -= amount;
    }
    T::from_usize(next)
}

impl<T, const MAXIMUM: usize> Rotate for NumericDie<T, MAXIMUM>
where
    T: Numeric + Add<Output = T> + Sub<Output = T>,
{
    #[allow(clippy::comparison_chain)]
    #[must_use]
    fn rotate(&self, amount: i8) -> Self {
        if amount == 0 {
            return self.clone();
        }
        let result = if amount > 0 {
            rotate_forward_usize::<T, MAXIMUM>(amount.unsigned_abs() as usize, self.0.as_usize())
        } else {
            rotate_backward_usize::<T, MAXIMUM>(amount.unsigned_abs() as usize, self.0.as_usize())
        };
        unsafe { Self::from_unchecked(result) }
    }
}

impl<T, const MAXIMUM: usize> RotateMut for NumericDie<T, MAXIMUM>
where
    T: Numeric + Debug + Add<Output = T> + Sub<Output = T>,
{
    fn rotate_mut(&mut self, amount: i8) {
        if amount == 0 {
            return;
        }
        let result = if amount > 0 {
            rotate_forward_usize::<T, MAXIMUM>(amount.unsigned_abs() as usize, self.0.as_usize())
        } else {
            rotate_backward_usize::<T, MAXIMUM>(amount.unsigned_abs() as usize, self.0.as_usize())
        };
        self.0 = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_die_is_clone() {
        #[allow(clippy::redundant_clone)]
        fn clone<T>(die: T) -> T
        where
            T: Clone,
        {
            die.clone()
        }

        let d4_2 = clone(D4::from(2));
        assert_eq!(d4_2.value(), 2);
    }

    #[test]
    fn numeric_die_is_debug() {
        let d4_2 = D4::from(2);
        assert_eq!(format!("{:?}", d4_2), "D4:2");
    }

    #[test]
    fn numeric_die_is_default() {
        let d4_1: D4 = Default::default();
        assert_eq!(d4_1.value(), 1);
    }

    #[test]
    fn numeric_die_is_eq() {
        let a = D4::from(2);
        let b = D4::from(2);
        assert_eq!(a, b);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn numeric_die_from_out_of_bounds_minimum() {
        D4::from(0);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn numeric_die_from_out_of_bounds_maximum() {
        D4::from(5);
    }

    #[test]
    fn numeric_die_step_next() {
        let d4_2 = D4::from(2);
        let d4_3 = d4_2.next();

        assert_eq!(d4_3.value(), 3);
    }

    #[test]
    fn numeric_die_step_next_wrap() {
        let d4_4 = D4::from(4);
        let d4_1 = d4_4.next();

        assert_eq!(d4_1.value(), 1);
    }

    #[test]
    fn numeric_die_step_back() {
        let d4_2 = D4::from(2);
        let d4_3 = d4_2.back();

        assert_eq!(d4_3.value(), 1);
    }

    #[test]
    fn numeric_die_step_back_wrap() {
        let d4_1 = D4::from(1);
        let d4_4 = d4_1.back();

        assert_eq!(d4_4.value(), 4);
    }

    #[test]
    fn numeric_die_step_next_mut() {
        let mut d4 = D4::from(2);
        d4.next_mut();

        assert_eq!(d4.value(), 3);
    }

    #[test]
    fn numeric_die_step_next_mut_wrap() {
        let mut d4 = D4::from(4);
        d4.next_mut();

        assert_eq!(d4.value(), 1);
    }

    #[test]
    fn numeric_die_step_back_mut() {
        let mut d4 = D4::from(2);
        d4.back_mut();

        assert_eq!(d4.value(), 1);
    }

    #[test]
    fn numeric_die_step_back_mut_wrap() {
        let mut d4 = D4::from(1);
        d4.back_mut();

        assert_eq!(d4.value(), 4);
    }

    #[test]
    fn numeric_die_rotate_none() {
        let d4_2 = D4::from(2);
        let d4_2 = d4_2.rotate(0);

        assert_eq!(d4_2.value(), 2);
    }

    #[test]
    fn numeric_die_rotate_mut_none() {
        let mut d4_2 = D4::from(2);
        d4_2.rotate_mut(0);

        assert_eq!(d4_2.value(), 2);
    }

    #[test]
    fn numeric_die_rotate_next() {
        let d4_2 = D4::from(2);
        let d4_3 = d4_2.rotate(1);

        assert_eq!(d4_3.value(), 3);
    }

    #[test]
    fn numeric_die_rotate_next_wrap() {
        let d4_2 = D4::from(2);
        let d4_1 = d4_2.rotate(3);

        assert_eq!(d4_1.value(), 1);
    }

    #[test]
    fn numeric_die_rotate_back() {
        let d4_2 = D4::from(2);
        let d4_1 = d4_2.rotate(-1);

        assert_eq!(d4_1.value(), 1);
    }

    #[test]
    fn numeric_die_rotate_back_wrap() {
        let d4_2 = D4::from(2);
        let d4_3 = d4_2.rotate(-3);

        assert_eq!(d4_3.value(), 3);
    }

    #[test]
    fn numeric_die_rotate_next_mut() {
        let mut d4 = D4::from(2);
        d4.rotate_mut(3);

        assert_eq!(d4.value(), 1);
    }

    #[test]
    fn numeric_die_rotate_next_mut_wrap() {
        let mut d4 = D4::from(2);
        d4.rotate_mut(-1);

        assert_eq!(d4.value(), 1);
    }

    #[test]
    fn numeric_die_rotate_back_mut() {
        let mut d4 = D4::from(2);
        d4.rotate_mut(-1);

        assert_eq!(d4.value(), 1);
    }

    #[test]
    fn numeric_die_rotate_back_mut_wrap() {
        let mut d4 = D4::from(2);
        d4.rotate_mut(-3);

        assert_eq!(d4.value(), 3);
    }
}
