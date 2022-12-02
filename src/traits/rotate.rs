/// A trait that creates elements by use of _step_ functions, i.e. seeking forward or backward.
pub trait Step {
    /// Steps _forward_ logically, for whatever that means, returning rotated by 1.
    #[must_use]
    fn next(&self) -> Self;

    /// Steps _backward_ logically, for whatever that means, returning rotated by -1.
    #[must_use]
    fn back(&self) -> Self;
}

/// A trait that mutates state by use of _step_ functions, i.e. seeking forward or backward.
pub trait StepMut {
    /// Steps _forward_ logically, for whatever that means, rotating by 1.
    fn next_mut(&mut self);

    /// Steps _backward_ logically, for whatever that means, rotating by -1.
    fn back_mut(&mut self);
}

/// A trait that can create rotated (forwards or backwards) elements given a number.
pub trait Rotate
where
    Self: Clone + Step,
{
    /// Rotates either forwards or backwards, based on the given amount.
    ///
    /// # Performance
    ///
    /// The default implementation is naive, and uses a loop combined with `next` and `back`, or
    /// `O(n)` where n is the `amount`. Where possible, replace this method with one that can seek
    /// directly and has better runtime and memory performance.
    #[allow(clippy::comparison_chain)]
    #[must_use]
    fn rotate(&self, amount: i8) -> Self {
        let mut next = self.clone();
        if amount == 0 {
            return next;
        }
        let forwards = amount > 0;
        let mut amount = amount.abs();
        while amount > 0 {
            next = if forwards { next.next() } else { next.back() };
            amount -= 1;
        }
        next
    }
}

/// A trait that can rotate (mutating; forwards or backwards) elements given a number.
pub trait RotateMut
where
    Self: StepMut,
{
    /// Rotates either forwards or backwards, based on the given amount.
    ///
    /// # Performance
    ///
    /// The default implementation is naive, and uses a loop combined with `next` and `back`, or
    /// `O(n)` where n is the `amount`. Where possible, replace this method with one that can seek
    /// directly and has better runtime and memory performance.
    #[allow(clippy::comparison_chain)]
    fn rotate_mut(&mut self, amount: i8) {
        if amount == 0 {
            return;
        }
        let forwards = amount > 0;
        let mut amount = amount.abs();
        while amount > 0 {
            if forwards {
                self.next_mut();
            } else {
                self.back_mut();
            };
            amount -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct FakeDie(i8);

    impl Step for FakeDie {
        fn next(&self) -> Self {
            FakeDie(self.0 + 1)
        }

        fn back(&self) -> Self {
            FakeDie(self.0 - 1)
        }
    }

    impl StepMut for FakeDie {
        fn next_mut(&mut self) {
            self.0 += 1
        }

        fn back_mut(&mut self) {
            self.0 -= 1
        }
    }

    impl Rotate for FakeDie {}
    impl RotateMut for FakeDie {}

    #[test]
    fn rotate_impl_none() {
        let d = FakeDie(0);
        let r = d.rotate(0);

        assert_eq!(r.0, 0);
    }

    #[test]
    fn rotate_impl_forwards() {
        let d = FakeDie(0);
        let r = d.rotate(2);

        assert_eq!(r.0, 2);
    }

    #[test]
    fn rotate_impl_backwards() {
        let d = FakeDie(0);
        let r = d.rotate(-2);

        assert_eq!(r.0, -2);
    }

    #[test]
    fn rotate_mut_impl_none() {
        let mut d = FakeDie(0);
        d.rotate_mut(0);

        assert_eq!(d.0, 0);
    }

    #[test]
    fn rotate_mut_impl_forwards() {
        let mut d = FakeDie(0);
        d.rotate_mut(2);

        assert_eq!(d.0, 2);
    }

    #[test]
    fn rotate_mut_impl_backwards() {
        let mut d = FakeDie(0);
        d.rotate_mut(-2);

        assert_eq!(d.0, -2);
    }
}
