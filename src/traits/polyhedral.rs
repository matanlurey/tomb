/// A trait indicating a finite number of _sides_ or ordered potential values.
///
/// Polyhedrals indicate that:
///
/// 1. A type has a fixed number of potential values (sides).
/// 2. The values are listed, in order, where the first value is the default value (side).
/// 3. A created object is always pointing at a valid value (side).
///
/// # Example
///
/// ```
/// # use tomb::traits::Polyhedral;
/// #[derive(Clone, Copy)]
/// enum Color {
///     Red,
///     Blue,
/// }
///
/// // https://starwars.fandom.com/wiki/Chance_cube
/// struct ChanceCube(usize);
///
/// unsafe impl Polyhedral<6> for ChanceCube {
///     type Side = Color;
///
///     const SIDES: [Color; 6] = [
///         Color::Red,
///         Color::Blue,
///         Color::Red,
///         Color::Blue,
///         Color::Red,
///         Color::Blue,
///     ];
///
///     unsafe fn from_unchecked(position: usize) -> Self {
///         assert!(position < Self::SIDES.len());
///         ChanceCube(position)
///     }
///
///     fn position(&self) -> usize {
///         self.0
///     }
/// }
/// ```
///
/// # Safety
///
/// This is an unsafe trait, and incorrectly implementing is undefined behavior.
///
/// Informally, by implementing it, you're asserting that:
///
/// 1. [`Polyhedral::from_unchecked`] _panics_ if provided a value other than `0..LENGTH`.
/// 2. [`Polyhedral::position`] always provides a value in the range `0..LENGTH`.
#[allow(clippy::len_without_is_empty)]
pub unsafe trait Polyhedral<const LENGTH: usize> {
    /// Underlying value type, that represents a _side_ of the polyhedral.
    ///
    /// While it is _recommended_ for this value to be trivially copyable, only `Clone` is required.
    type Side: Clone;

    /// Sides of the polyhedral.
    ///
    /// The first element is assumed to be the default value.
    ///
    /// Values do not need to be unique, and other than the first element order is not significant.
    const SIDES: [Self::Side; LENGTH];

    /// Creates a new default polyhedral.
    fn new() -> Self
    where
        Self: Sized,
    {
        unsafe { Self::from_unchecked(0) }
    }

    /// Creates a new polyhedral, setting the side to the given position.
    ///
    /// # Safety
    ///
    /// This method is delegated to the implementation, and **must panic** if >= `LENGTH`.
    ///
    /// Suggested implementation:
    /// ```
    /// # struct X;
    /// # impl X {
    /// # const SIDES: [(); 0] = [];
    /// unsafe fn from_unchecked(position: usize) -> Self {
    ///     assert!(position < Self::SIDES.len());
    ///     /* ... */
    ///     # unreachable!()
    /// }
    /// # }
    /// ```
    unsafe fn from_unchecked(position: usize) -> Self;

    /// Returns the total number of sides of the polyhedral.
    fn len(&self) -> usize {
        LENGTH
    }

    /// Returns the current position of the polyhedral, which is always within `0..LENGTH`.
    fn position(&self) -> usize;

    /// Returns the value currently being pointed at by the polyhedral.
    fn value(&self) -> Self::Side {
        Self::SIDES[self.position()].clone()
    }

    /// Sets the current position of the polyhedral.
    ///
    /// # Panics
    ///
    /// If the position is out of bounds.
    fn set(&mut self, position: usize) {
        assert!(position < self.len());
    }
}
