use crate::traits::{Rotate, RotateMut, Step, StepMut};

/// A die that has a known and fixed set of values, and a position that points at the current value.
///
/// When you think of dice, [`NumericDie`] is both _simpler to use_ and _more typical_ (a range of
/// numbers). However, if that number will be mapped back to a non-numeric value, for example either
/// a discrete value, an enum, or to have a weighted effect, in steps `SliceDie`.
///
/// # Examples
///
/// ```
/// # use tomb::items::SliceDie;
/// // The lifetime scope of these referneces will often be static, but not always.
/// const GRADES: [char; 5] = ['A', 'B', 'C', 'D', 'F'];
///
/// SliceDie::from(&GRADES);
/// ```
#[derive(Clone, Copy)]
pub struct SliceDie<'a, T, const LENGTH: usize> {
    position: usize,
    elements: &'a [T; LENGTH],
}

impl<'a, T, const LENGTH: usize> SliceDie<'a, T, LENGTH> {
    /// Creates a new slice die from the given possible sides of the die.
    ///
    /// The current position is set to `0`, or the first element in the slice.
    ///
    /// # Panics
    ///
    /// If the given slice is empty.
    pub const fn new(elements: &'a [T; LENGTH]) -> Self {
        assert!(LENGTH > 0);
        Self {
            elements,
            position: 0,
        }
    }

    /// Creates a new die starting at the given `value`.
    ///
    /// # Safety
    ///
    /// The value is _not_ checked for bounds correctness, and could cause undefined behavior.
    unsafe fn from_unchecked(position: usize, elements: &'a [T; LENGTH]) -> Self {
        Self { elements, position }
    }

    /// Returns the current position within the die, between `0..self.len()`.
    pub const fn position(&self) -> usize {
        self.position
    }

    /// Returns a reference to the sides within the die.
    pub const fn sides(&self) -> &'a [T; LENGTH] {
        self.elements
    }

    /// Returns a reference to the currently faced value.
    ///
    /// This method is always equivalent to `self.elements()[self.position()]`.
    pub const fn value(&self) -> &T {
        &self.elements[self.position]
    }
}

impl<'a, T, const LENGTH: usize> From<&'a [T; LENGTH]> for SliceDie<'a, T, LENGTH> {
    /// Converts a slice of elements into a die of the same length.
    ///
    /// The current position is set to `0`, or the first element in the slice.
    ///
    /// # Panics
    ///
    /// If the given slice is empty.
    fn from(elements: &'a [T; LENGTH]) -> Self {
        Self::new(elements)
    }
}

impl<'a, T, const MAXIMUM: usize> Step for SliceDie<'a, T, MAXIMUM> {
    /// Rotates the die forward by one element.
    ///
    /// If the value would have surpassed the maximum, it returns back to the first element.
    fn next(&self) -> Self {
        let mut next = self.position + 1;
        if next == MAXIMUM {
            next = 0;
        }
        unsafe { Self::from_unchecked(next, self.elements) }
    }

    /// Rotates the die backwards by one element.
    ///
    /// If the value would have surpassed the minimum, it returns back to the last element.
    fn back(&self) -> Self {
        let mut next = self.position;
        if next == 0 {
            next = MAXIMUM - 1;
        } else {
            next -= 1;
        }
        unsafe { Self::from_unchecked(next, self.elements) }
    }
}

impl<'a, T, const MAXIMUM: usize> StepMut for SliceDie<'a, T, MAXIMUM> {
    /// Rotates the die forward by one element.
    ///
    /// If the value would have surpassed the maximum, it returns back to the first element.
    fn next_mut(&mut self) {
        let mut next = self.position + 1;
        if next == MAXIMUM {
            next = 0;
        }
        self.position = next;
    }

    /// Rotates the die backwards by one element.
    ///
    /// If the value would have surpassed the minimum, it returns back to the last element.
    fn back_mut(&mut self) {
        let mut next = self.position;
        if next == 0 {
            next = MAXIMUM - 1;
        } else {
            next -= 1;
        }
        self.position = next;
    }
}

fn rotate_forward_usize<const MAXIMUM: usize>(position: usize, amount: usize) -> usize {
    debug_assert!(amount > 0);
    (position + amount) % MAXIMUM
}

fn rotate_backward_usize<const MAXIMUM: usize>(position: usize, amount: i8) -> usize {
    let current = position as i8;
    let rotated = current - amount;
    if rotated >= 0 {
        return rotated.unsigned_abs() as usize;
    }
    let size = MAXIMUM as i8;
    let rotated = rotated % size + size;
    debug_assert!(rotated >= 0);
    rotated as usize
}

impl<'a, T, const MAXIMUM: usize> Rotate for SliceDie<'a, T, MAXIMUM>
where
    T: Clone,
{
    #[allow(clippy::comparison_chain)]
    fn rotate(&self, amount: i8) -> Self {
        if amount == 0 {
            return self.clone();
        }
        let position = if amount > 0 {
            rotate_forward_usize::<MAXIMUM>(self.position, amount.unsigned_abs() as usize)
        } else {
            rotate_backward_usize::<MAXIMUM>(self.position, amount)
        };
        unsafe { Self::from_unchecked(position, self.elements) }
    }
}

impl<'a, T, const MAXIMUM: usize> RotateMut for SliceDie<'a, T, MAXIMUM> {
    fn rotate_mut(&mut self, amount: i8) {
        if amount == 0 {
            return;
        }
        let position = if amount > 0 {
            rotate_forward_usize::<MAXIMUM>(self.position, amount.unsigned_abs() as usize)
        } else {
            rotate_backward_usize::<MAXIMUM>(self.position, amount)
        };
        self.position = position;
    }
}
