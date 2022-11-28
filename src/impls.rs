//! Helper functions that can be used to implement your own types just like `tomb`.

/// Implements [`crate::Polyhedral::next`] where `V` is a unique value from a slice of values.
///
/// # Panics
///
/// If `current` is out of bounds.
pub fn next_slice<T, const SIZE: usize>(current: usize, all: &[T; SIZE]) -> T
where
    T: Clone,
{
    assert!(current < SIZE);
    let next = if current + 1 == SIZE { 0 } else { current + 1 };
    all[next].clone()
}

/// Implements [`crate::Polyhedral::rotate`] where `V` is a unique value from a slice of values.
///
/// # Panics
///
/// If `current` is out of bounds.
#[allow(clippy::comparison_chain)]
pub fn rotate_slice<T, const SIZE: usize>(current: usize, all: &[T; SIZE], amount: i8) -> T
where
    T: Clone,
{
    assert!(current < SIZE);
    if amount == 0 {
        all[current].clone()
    } else if amount < 0 {
        rotate_backward(current, all, amount.unsigned_abs() as usize)
    } else {
        rotate_forward(current, all, amount as usize)
    }
}

fn rotate_forward<T, const SIZE: usize>(current: usize, all: &[T; SIZE], amount: usize) -> T
where
    T: Clone,
{
    all[((current + amount) % SIZE)].clone()
}

fn rotate_backward<T, const SIZE: usize>(current: usize, all: &[T; SIZE], amount: usize) -> T
where
    T: Clone,
{
    let current = current as i8;
    let amount = amount as i8;
    let rotated = current - amount;

    if rotated >= 0 {
        return all[rotated.unsigned_abs() as usize].clone();
    }

    let size = SIZE as i8;
    let rotated = rotated % size + size;
    debug_assert!(rotated >= 0);

    all[rotated as usize].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRADES: [char; 5] = ['A', 'B', 'C', 'D', 'F'];

    #[test]
    fn rotate_forward() {
        // A -> C
        let result = rotate_slice(0, &GRADES, 2);
        assert_eq!(result, 'C');
    }

    #[test]
    fn rotate_backward() {
        // C -> A
        let result = rotate_slice(2, &GRADES, -2);
        assert_eq!(result, 'A');
    }

    #[test]
    fn rotate_forward_loop() {
        // C -> A
        let result = rotate_slice(2, &GRADES, 3);
        assert_eq!(result, 'A');
    }

    #[test]
    fn rotate_backward_loop() {
        // A -> C
        let result = rotate_slice(0, &GRADES, -3);
        assert_eq!(result, 'C');
    }
}
