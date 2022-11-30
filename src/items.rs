//! Contains "physical" objects or objects that act on physical objects.
//!
//! A typical app will create and manage one or more _die_, typically [`NumericDie`] which are
//! simpler than [`SliceDie`], and often one of the convenience type such as [`D6`], and either
//! manually work with the dice or use a (recommended) _roller_, typically
//! [`crate::items::RngRoller`].
//!
//! # Examples
//!
//! ```
//! // A static seed is provided in order to make this example predictable.
//! use fastrand::Rng;
//! use tomb::items::{D6, RngRoller};
//! use tomb::traits::{Roll, RollMut};
//!
//! let roller = RngRoller::from(Rng::with_seed(7194422452970863838));
//!
//! // Immutable objects.
//! let d6 = D6::new();
//! let rd = roller.roll(d6);
//! assert_eq!(rd.value(), 3);
//! ```

mod dice;
mod roller;

pub use dice::*;
pub use roller::*;

#[cfg(test)]
mod tests {
    use fastrand::Rng;

    use crate::items::{RngRoller, D6};
    use crate::traits::Roll;

    #[test]
    fn roll() {
        let roller = RngRoller::from(Rng::with_seed(7194422452970863838));

        let d6 = D6::new();
        let rd = roller.roll(d6);
        assert_eq!(rd.value(), 3);
    }
}
