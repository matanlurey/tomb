//! Contains behaviors physical objects can declare.
//!
//! Some frequently used traits will include:
//!
//! - [`Numeric`] allows flexibility when defining _numeric_ die.
//! - [`Polyhedral`] defines objects with a known number of sides.
//! - [`Rotate`] and [`Roll`] create or mutate objects with multiple sides.
//!
//! For most users, the traits exposed in [`crate`] are sufficient.

mod numeric;
mod polyhedral;
mod roll;
mod rotate;

pub use numeric::*;
pub use polyhedral::*;
pub use roll::*;
pub use rotate::*;
