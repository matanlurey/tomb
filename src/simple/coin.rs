use crate::*;

/// _Just a simple coin, trying to make its way in the universe._
///
/// This coin represents either:
/// - [`CoinFacing::Heads`]
/// - [`CoinFacing::Tails`]
///
/// ... and is a great albeit simple example of using this library to define a [`Polyhedral`].
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct SimpleCoin {
    value: CoinFacing,
}

/// Base implementation for our simple coin, which has no knowledge (yet) of the `tomb` crate.
impl SimpleCoin {
    /// Creates a new coin with the given value set.
    pub const fn new(value: CoinFacing) -> Self {
        Self { value }
    }

    /// Creates a new coin with [`CoinFacing::Heads`] set.
    pub const fn heads() -> Self {
        Self::new(CoinFacing::Heads)
    }

    /// Creates a new coin with [`CoinFacing::Tails`] set.
    pub const fn tails() -> Self {
        Self::new(CoinFacing::Tails)
    }

    /// Returns whether the coin faces heads.
    pub fn is_heads(&self) -> bool {
        self.value == CoinFacing::Heads
    }

    /// Returns whether the coin faces tails.
    pub fn is_tails(&self) -> bool {
        self.value == CoinFacing::Tails
    }
}

/// Possible facings for a [`SimpleCoin`].
///
/// We could have represented this as a `bool`, but we lose clarity of what `true` or `false` means.
///
/// See also: [`SimpleCoin::is_heads`] and [`SimpleCoin::is_tails`].
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum CoinFacing {
    #[default]
    Heads,
    Tails,
}

impl Polyhedral<CoinFacing, 2> for SimpleCoin {
    unsafe fn as_unchecked(&self, current: CoinFacing) -> Self {
        Self::new(current)
    }

    fn get(&self) -> CoinFacing {
        self.value
    }

    fn sides(&self) -> &[CoinFacing; 2] {
        const FACES: [CoinFacing; 2] = [CoinFacing::Heads, CoinFacing::Tails];
        &FACES
    }

    fn current(&self) -> usize {
        self.value as usize
    }

    fn is_valid(&self, _: &CoinFacing) -> bool {
        true
    }

    unsafe fn set_unchecked(&mut self, value: CoinFacing) -> CoinFacing {
        self.value = value;
        value
    }
}

impl Coin<CoinFacing> for SimpleCoin {
    fn swap(&self) -> Self {
        self.next()
    }

    fn swap_mut(&mut self) -> CoinFacing {
        self.next_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_coin_heads() {
        let coin = SimpleCoin::heads();
        assert!(coin.is_heads());
    }

    #[test]
    fn simple_coin_tails() {
        let coin = SimpleCoin::tails();
        assert!(coin.is_tails());
    }

    #[test]
    fn simple_coin_next() {
        let coin = SimpleCoin::heads();
        let tails = coin.next();
        assert!(tails.is_tails());
        let heads = tails.next();
        assert!(heads.is_heads());
    }

    #[test]
    fn simple_coin_swap() {
        let coin = SimpleCoin::heads();
        let tails = coin.swap();
        assert!(tails.is_tails());
        let heads = tails.swap();
        assert!(heads.is_heads());
    }

    #[test]
    fn simple_coin_rotate() {
        let coin = SimpleCoin::heads();

        let tails = coin.rotate(1);
        assert!(tails.is_tails());

        let tails = coin.rotate(-1);
        assert!(tails.is_tails());

        let tails = coin.rotate(-5);
        assert!(tails.is_tails());
    }

    #[test]
    fn simple_coin_set() {
        let mut coin = SimpleCoin::heads();
        coin.set(CoinFacing::Tails);
        assert!(coin.is_tails());
    }

    #[test]
    fn simple_coin_swap_mut() {
        let mut coin = SimpleCoin::heads();
        coin.swap_mut();
        assert!(coin.is_tails());
    }

    #[test]
    fn simple_coin_rotate_mut() {
        let mut coin = SimpleCoin::heads();
        coin.rotate_mut(3);
        assert!(coin.is_tails());
    }

    #[test]
    fn simple_coin_next_mut() {
        let mut coin = SimpleCoin::heads();
        coin.next_mut();
        assert!(coin.is_tails());
    }
}
