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
//!
//! # Examples
//!
//! Creating and rolling a D20
//!
//! ```
//! // Optional dependency, exclude to implement your own RNG.
//! use fastrand::Rng;
//!
//! // It is possible to define your own dice, rollers, and to use immutable die as well!
//! use tomb::{D20, RngRoller, RollMut};
//!
//! // Pre-defined seed so the result of this example is predictable.
//! let roller = RngRoller::from(Rng::with_seed(7194422452970863838));
//! let mut d20 = D20::new();
//!
//! roller.roll_mut(&mut d20);
//! assert_eq!(d20.value(), 10);
//! ```

pub mod items;
pub mod traits;

pub use items::{NumericDie, RngRoller, D20, D6};
pub use traits::{Roll, RollMut};
