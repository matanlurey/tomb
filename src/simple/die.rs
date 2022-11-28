use std::ops::RangeInclusive;

use crate::*;

macro_rules! simple_die {
    ($name:ident, $sides:expr, $arr:expr) => {
        #[doc = concat!("A a simple numerical ", $sides, "-sided die.")]
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
        pub struct $name {
            value: u8,
        }

        impl $name {
            pub const fn new() -> Self {
                Self { value: 1 }
            }
        }

        impl Polyhedral<u8, $sides> for $name {
            unsafe fn as_unchecked(&self, value: u8) -> Self {
                Self { value }
            }

            fn current(&self) -> usize {
                self.value as usize - 1
            }

            fn sides(&self) -> &[u8; $sides] {
                const SIDES: [u8; $sides] = $arr;
                &SIDES
            }

            fn get(&self) -> u8 {
                self.value
            }

            unsafe fn set_unchecked(&mut self, value: u8) -> u8 {
                self.value = value;
                value
            }

            fn is_valid(&self, value: &u8) -> bool {
                const RANGE: RangeInclusive<u8> = 1..=$sides;
                RANGE.contains(value)
            }
        }
    };
}

/* TODO: Further macro-ize this. */

simple_die!(SimpleD4, 4, [1, 2, 3, 4]);
simple_die!(SimpleD6, 6, [1, 2, 3, 4, 5, 6]);
simple_die!(SimpleD8, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
simple_die!(SimpleD10, 10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
simple_die!(SimpleD12, 12, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
#[rustfmt::skip]
simple_die!(SimpleD20, 20, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
