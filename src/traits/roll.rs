use super::{Polyhedral, Rotate, RotateMut};

pub trait Roll {
    /// Rotates an entity randomly, returning the result, where random is defined elsewhere.
    #[must_use]
    fn roll<T>(&self, rotate: T) -> T
    where
        T: Rotate + Polyhedral;
}

pub trait RollMut {
    /// Rotates an entity randomly, mutating the entity, where random is defined elsewhere.
    fn roll_mut<T>(&self, rotate: T)
    where
        T: RotateMut + Polyhedral;
}
