use fastrand::Rng;

use crate::traits::{Polyhedral, Roll, RollMut, Rotate, RotateMut};

/// Declares that it rolls entities, but does nothing.
///
/// # Examples
///
/// ```
/// use tomb::items::{D6, NopRoller};
/// use tomb::traits::RollMut;
///
/// let roller = NopRoller;
/// let mut d6 = D6::new();
/// assert_eq!(d6.value(), 1);
///
/// // An arbitrary number of times, just to show it never changes the die.
/// for _ in 0..10 {
///     roller.roll_mut(&mut d6);
///     assert_eq!(d6.value(), 1);
/// }
/// ```
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
    fn roll_mut<T>(&self, _rotate: &mut T)
    where
        T: RotateMut,
    {
        /* Intentionally left blank. */
    }
}

/// Rolls entities using the `fastrand` crate.
///
/// # Examples
///
/// ```
/// use fastrand::Rng;
/// use tomb::items::{D6, RngRoller};
/// use tomb::traits::RollMut;
///
/// // Arbirtary number to make the example result predictable.
/// let rng = Rng::with_seed(7194422452970863838);
/// let roller = RngRoller::from(rng);
/// let mut d6 = D6::new();
/// assert_eq!(d6.value(), 1);
///
/// roller.roll_mut(&mut d6);
/// assert_eq!(d6.value(), 3);
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
    fn roll_mut<T>(&self, rotate: &mut T)
    where
        T: Polyhedral + RotateMut,
    {
        let sides = T::sides();
        let range = 0..sides;
        let amount = self.0.usize(range);

        rotate.rotate_mut(amount as i8);
    }
}
