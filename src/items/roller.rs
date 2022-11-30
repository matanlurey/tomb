use fastrand::Rng;

use crate::traits::{Polyhedral, Roll, RollMut, Rotate, RotateMut};

/// Declares that it rolls entities, but does nothing.
#[derive(Clone, Copy, Default)]
pub struct NopRoller;

impl NopRoller {
    /// Creates a NOP (no-op) roller that doesn't actually roll anything.
    pub const fn new() -> Self {
        Self
    }
}

impl Roll for NopRoller {
    fn roll<T>(&self, rotate: T) -> T
    where
        T: Rotate,
    {
        rotate
    }
}

impl RollMut for NopRoller {
    fn roll_mut<T>(&self, mut _rotate: T)
    where
        T: RotateMut,
    {
        /* Intentionally left blank. */
    }
}

/// Rolls entities using the `fastrand` crate.
#[cfg(feature = "fastrand")]
#[derive(Clone, Default)]
pub struct RngRoller(Rng);

impl RngRoller {
    /// Creates a new roller that creates a default RNG.
    pub fn new() -> Self {
        Self(Rng::new())
    }
}

impl From<Rng> for RngRoller {
    /// Creates a new roller that delegates to the given RNG.
    fn from(rng: Rng) -> Self {
        Self(rng)
    }
}

impl Roll for RngRoller {
    fn roll<T>(&self, rotate: T) -> T
    where
        T: Polyhedral + Rotate,
    {
        let sides = T::sides();
        let range = 0..sides;
        let amount = self.0.usize(range);
        rotate.rotate(amount as i8)
    }
}

impl RollMut for RngRoller {
    fn roll_mut<T>(&self, mut rotate: T)
    where
        T: Polyhedral + RotateMut,
    {
        let sides = T::sides();
        let range = 0..sides;
        let amount = self.0.usize(range);
        rotate.rotate_mut(amount as i8);
    }
}
