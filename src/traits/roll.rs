use super::{Polyhedral, Rotate, RotateMut};

/// A trait that creates new elements based off ones which [`Rotate`] and are [`Polyhedral`].
///
/// In practice, this is used to allow a die _roller_ in order to create a new (immutable) die
/// by picking a new side, randomly, of one of the possible dies. For example, for a D6, picking
/// between the values `1..=6`.
pub trait Roll {
    /// Rotates an entity randomly, returning the result, where random is defined elsewhere.
    #[must_use]
    fn roll<T>(&self, rotate: T) -> T
    where
        T: Rotate + Polyhedral;
}

/// A trait that mutates existing elements which [`RotateMut`] and are [`Polyhedral`].
///
/// In practice, this is used to allow a die _roller_ in order to create a new (immutable) die
/// by picking a new side, randomly, of one of the possible dies. For example, for a D6, picking
/// between the values `1..=6`.
pub trait RollMut {
    /// Rotates an entity randomly, mutating the entity, where random is defined elsewhere.
    fn roll_mut<T>(&self, rotate: &mut T)
    where
        T: RotateMut + Polyhedral;
}
