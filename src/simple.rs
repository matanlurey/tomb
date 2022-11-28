//! Simple implementations that can help explain the crate to first-time users.
//!
//! If you just need coins or some dice, and aren't looking to further customize, look no further.
//!
//! # Examples
//!
//! ## Coins
//!
//! ```
//! use tomb::Coin;
//! use tomb::simple::SimpleCoin;
//!
//! // Immutable
//! let heads = SimpleCoin::heads();
//! assert!(heads.is_heads());
//! let tails = heads.swap();
//! assert!(tails.is_tails());
//!
//! // Mutable
//! let mut coin = SimpleCoin::heads();
//! coin.swap_mut();
//! assert!(coin.is_tails());
//! ```
//!
//! ## Dice
//!
//! ```
//! use tomb::Polyhedral;
//! use tomb::simple::SimpleD6;
//!
//! // Immutable
//! let one = SimpleD6::new();
//! assert_eq!(one.get(), 1);
//! let two = one.next();
//! assert_eq!(two.get(), 2);
//!
//! // Mutable
//! let mut die = SimpleD6::new();
//! die.next_mut();
//! assert_eq!(die.get(), 2);
//! ```

mod coin;
mod die;

pub use coin::*;
pub use die::*;
