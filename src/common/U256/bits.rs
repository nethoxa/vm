use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::common::errors::CommonError;

use super::U256;

impl U256 {
	pub fn bit(
		&self,
		index: u64
	) -> Result<bool, CommonError> {
		if index >= Self::BITS {
			Err(CommonError::OutOfBoundsAccess)
		} else {
			let (word, bit) = (3 - index / 64, index % 64);
			let set = self.words[word as usize] & (1 << bit) != 0;

			Ok(set)
		}
	}

	pub fn set_bit(
		&mut self,
		index: u64,
		value: bool
	) -> Result<(), CommonError> {
		if index >= Self::BITS {
			Err(CommonError::OutOfBoundsAccess)
		} else {
			let (word, bit) = (3 - index / 64, index % 64);

			if value {
				self.words[word as usize] |= 1 << bit;
			} else {
				self.words[word as usize] &= !(1 << bit);
			}

			Ok(())
		}
	}

	pub fn byte(
		&self,
		index: u64
	) -> Result<u8, CommonError> {
		if index >= Self::BYTES {
			Err(CommonError::OutOfBoundsAccess)
		} else {
			let (word, byte) = (3 - index / 8, (index % 8) * 8);
			let result = self.words[word as usize] >> byte;

			Ok(result as u8)
		}
	}

	/// Mask to apply to the highest word to get the correct number of bits
	pub fn mask(bits: usize) -> u64 {
		if bits == 0 {
			0
		} else {
			let bits = bits % 64;

			if bits == 0 {
				u64::MAX
			} else {
				(1 << bits) - 1
			}
		}
	}
}

impl Shl for U256 {
	type Output = U256;

	fn shl(
		self,
		other: U256
	) -> U256 {
		let shift = other.words[3]; // if higher, then it defaults to 0
		if shift >= Self::BITS {
			U256::zero()
		} else {
			let (words, bits) = (shift / 64, shift % 64);
			let mut result = Self::zero();
			let mut carry = 0;

			for i in 0..Self::WORDS - words {
				let x = self.words[(3 - i) as usize];
				result.words[(3 - words - i) as usize] = (x << bits) | carry;
				carry = (x >> (64 - bits - 1)) >> 1;
			}

			result
		}
	}
}

impl ShlAssign for U256 {
	fn shl_assign(
		&mut self,
		other: U256
	) {
		*self = self.shl(other);
	}
}

impl Shr for U256 {
	type Output = U256;

	fn shr(
		self,
		other: U256
	) -> U256 {
		let shift = other.words[3]; // if higher, then it defaults to 0
		if shift >= Self::BITS {
			U256::zero()
		} else {
			let (words, bits) = (shift / 64, shift % 64);
			let mut result = Self::zero();
			let mut carry = 0;

			for i in 0..Self::WORDS - words {
				let x = self.words[(i) as usize];
				result.words[i as usize] = (x >> bits) | carry;
				carry = (x << (64 - bits - 1)) >> 1;
			}

			result
		}
	}
}

impl ShrAssign for U256 {
	fn shr_assign(
		&mut self,
		other: U256
	) {
		*self = self.shr(other);
	}
}

#[test]
fn test_bit_byte_getter() {
	let mut number = U256::zero();
	number.set_bit(33, true).unwrap();
	number.set_bit(102, true).unwrap();

	assert!(number.bit(32).unwrap() == false);
	assert!(number.bit(33).unwrap() == true);
	assert!(number.bit(34).unwrap() == false);

	assert!(number.bit(101).unwrap() == false);
	assert!(number.bit(102).unwrap() == true);
	assert!(number.bit(103).unwrap() == false);

	let number_as_u128 = u128::from(number);
	assert!(number_as_u128 == u128::pow(2, 102) + u128::pow(2, 33));

	assert!(number.byte(4).unwrap() == 2);
	assert!(number.byte(12).unwrap() == u8::pow(2, 6));
}

#[test]
fn test_shl() {
	let x = U256::from(20u64);
	let mut y = x << U256::from(3u64);

	assert!(y == U256::from(20 * 8 as u64));

	y = x << U256::from(257u64);
	assert!(y == U256::zero());
}

#[test]
fn test_shr() {
	let x = U256::from(20u64);
	let mut y = x >> U256::from(3u64);

	assert!(y == U256::from(20 / 8 as u64));

	y = x >> U256::from(257u64);
	assert!(y == U256::zero());
}
