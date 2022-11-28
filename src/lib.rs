//! Tomb is a minimal crate that provides dice rolling mechanisms for games.
//!
//! Why use `tomb`:
//! - The name is great (short for _tombstones_, another name for _dice_)
//! - Requires close to no depdendencies (an optional one for the `fastrand` crate)
//! - Fully tested with generous example code
//!
//! One way to think about `tomb` is as a minimal _headless_ [tabletop simulator][].
//!
//! [tabletop simulator]: https://www.tabletopsimulator.com/

pub mod impls;
pub mod simple;

#[cfg(feature = "fastrand")]
pub use fastrand::Rng;

/// A 2-sided object.
pub trait Coin<V> {
    /// Returns the coin with the value set to the "opposite" side.
    fn swap(&self) -> Self;

    /// Updates the coin with the value set to the "opposite" side.
    ///
    /// Returns the currently set value.
    fn swap_mut(&mut self) -> V;
}

/// A multi-sided object with a fixed amount of valid _faces_ or values `V`.
pub trait Polyhedral<V, const SIDES: usize>
where
    V: Clone + PartialEq,
{
    /// Returns a new polyhedral that represents a `current` value (facing).
    ///
    /// # Safety
    ///
    /// There is no check performed that current indeed _is_ a valid facing, for example it could
    /// be possible to have a value of `7` represented on a D6, which is otherwise undefined
    /// behavior.
    ///
    /// In cases where `V` is well-defined (e.g. an enum), this operation is _always_ safe.
    unsafe fn as_unchecked(&self, current: V) -> Self;

    /// Returns a polyhedral with the value set to the "next" side.
    ///
    /// For a coin, this would potentially return a tails-facing coin from a heads-facing one.
    /// For a dice, this would potentiallu return a 5-facing coin from a 4-facing one.
    fn next(&self) -> Self
    where
        Self: Sized,
    {
        let current = self.current();
        let sides = self.sides();
        let next = impls::next_slice(current, sides);
        unsafe { self.as_unchecked(next) }
    }

    /// Updates a polyhedral with the value set to the "next" side.
    ///
    /// For a coin, this would potentially return a tails-facing coin from a heads-facing one.
    /// For a dice, this would potentiallu return a 5-facing coin from a 4-facing one.
    ///
    /// Returns the newly set value.
    fn next_mut(&mut self) -> V
    where
        Self: Sized,
    {
        let current = self.current();
        let sides = self.sides();
        let next = impls::next_slice(current, sides);
        unsafe { self.set_unchecked(next) }
    }

    /// Returns a polyhedral with the value moved a finite number of times.
    fn rotate(&self, by: i8) -> Self
    where
        Self: Sized,
    {
        let current = self.current();
        let sides = self.sides();
        let next = impls::rotate_slice(current, sides, by);
        unsafe { self.as_unchecked(next) }
    }

    /// Updates a polyhedral with the value moved a finite number of times.
    ///
    /// Returns the newly set value.
    fn rotate_mut(&mut self, by: i8) -> V
    where
        Self: Sized,
    {
        let current = self.current();
        let sides = self.sides();
        let next = impls::rotate_slice(current, sides, by);
        unsafe { self.set_unchecked(next) }
    }

    /// Returns the polyhedral as the numeric _side_ represented by the current value.
    fn current(&self) -> usize;

    /// Returns a slice of all possible sides that could be represented as a value.
    ///
    /// The sides are expected to be in a logical order, for example:
    /// ```
    /// # struct X;
    /// # impl X {
    /// // Example of the sides of a D6
    /// fn sides(&self) -> &[u8; 6] {
    ///     const SIDES: [u8; 6] = [1, 2, 3, 4, 5, 6];
    ///     &SIDES
    /// }
    /// # }
    /// ```
    fn sides(&self) -> &[V; SIDES];

    /// Returns the current value, or _face_, of the polyhedral.
    fn get(&self) -> V;

    /// Returns whether the given value can be used for [`Polyhedral::set`].
    ///
    /// # Performance
    ///
    /// By default, this is implemented as `self.sides().contains(&value)`, which is `O(n)`. For
    /// larger `n` counts or when value is well-constrained (i.e. for a D20, the value be within
    /// `1..=20`), **override** this default method:
    ///
    /// ```
    /// # const SIDES: u8 = 6;
    /// # struct X;
    /// # impl X {
    /// // Example of validating a D20
    /// fn is_valid(&self, value: &u8) -> bool {
    ///     (1..=SIDES).contains(value)
    /// }
    /// # }
    /// ```
    ///
    /// For implementations where value is well-defined (e.g. an enum), just return true:
    /// ```
    /// # use tomb::simple::CoinFacing;
    /// # struct X;
    /// # impl X {
    /// // Example of validating a coin
    /// fn is_valid(&self, _: &CoinFacing) -> bool {
    ///     true
    /// }
    /// # }
    /// ```
    fn is_valid(&self, value: &V) -> bool {
        self.sides().contains(value)
    }

    /// Sets the current value, or _face_, of the polyhedral.
    ///
    /// # Panics
    ///
    /// If `value` is out of bounds or otherwise invalid.
    fn set(&mut self, value: V) -> V {
        if !self.is_valid(&value) {
            panic!("Not a valid value")
        }
        unsafe { self.set_unchecked(value) }
    }

    /// Sets the current value, or _face_, of the polyhedral.
    ///
    /// # Safety
    ///
    /// There is no check performed that `value` indeed _is_ a valid facing, for example it could be
    /// possible to have a value of `7` represented on a D6, which is otherwise undefined behavior.
    ///
    /// In cases where `V` is well-defined (e.g. an enum), this operation is _always_ safe.
    unsafe fn set_unchecked(&mut self, value: V) -> V;
}
