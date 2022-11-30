/// Rotates by use of _step_ functions, i.e. seeking forward or backward one step at a time.
pub trait Step {
    /// Steps _forward_ logically, for whatever that means, returning rotated by 1.
    #[must_use]
    fn next(&self) -> Self;

    /// Steps _backward_ logically, for whatever that means, returning rotated by -1.
    #[must_use]
    fn back(&self) -> Self;
}

/// Mutates by stepping the position, i.e. seeking forwad or backward one step at a time.
pub trait StepMut {
    /// Steps _forward_ logically, for whatever that means, rotating by 1.
    fn next_mut(&mut self);

    /// Steps _backward_ logically, for whatever that means, rotating by -1.
    fn back_mut(&mut self);
}

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
