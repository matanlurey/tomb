/// A trait that provides a known number of sides for a multi-sided element.
pub trait Polyhedral {
    /// Returns the number of sides present.
    fn sides() -> usize;
}
