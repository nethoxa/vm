pub mod add;
pub mod and;
pub mod bits;
pub mod div;
pub mod exp;
pub mod from;
pub mod mul;
pub mod neg;
pub mod not;
pub mod or;
pub mod root;
pub mod to;
pub mod xor;

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
/// Little endian 256-bits unsigned integer
pub struct U256 {
	pub words: [u64; 4]
}

impl U256 {
	pub const BITS: u64 = 256;
	pub const BYTES: u64 = 32;
	pub const MAX: U256 = U256 {
		words: [u64::MAX; 4]
	};
	pub const WORDS: u64 = 4;

	pub fn zero() -> U256 {
		U256 {
			words: [0; 4]
		}
	}

	pub fn one() -> U256 {
		U256 {
			words: [0, 0, 0, 1]
		}
	}

	pub fn new(words: [u64; 4]) -> U256 {
		U256 {
			words
		}
	}
}
