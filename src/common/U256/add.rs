use std::ops::{Add, AddAssign, Sub, SubAssign};

use super::U256;

impl U256 {
	pub fn overflowing_add(
		self,
		other: U256
	) -> (U256, bool) {
		let mut result = [0; 4];
		let mut overflow = false;

		for i in 0..4 {
			let (v, o1) = self.words[3 - i].overflowing_add(other.words[3 - i]);
			let (v, o2) = v.overflowing_add(overflow as u64);

			result[3 - i] = v;
			overflow = o1 | o2;
		}

		(U256::new(result), overflow)
	}

	pub fn overflowing_sub(
		self,
		other: U256
	) -> (U256, bool) {
		let mut result = [0; 4];
		let mut overflow = false;

		for i in 0..4 {
			let (v, o1) = self.words[3 - i].overflowing_sub(other.words[3 - i]);
			let (v, o2) = v.overflowing_sub(overflow as u64);

			result[3 - i] = v;
			overflow = o1 | o2;
		}

		(U256::new(result), overflow)
	}

	pub fn overflowing_neg(self) -> (U256, bool) { Self::zero().overflowing_sub(self) }
}

impl Add for U256 {
	type Output = U256;

	fn add(
		self,
		other: U256
	) -> U256 {
		self.overflowing_add(other).0
	}
}

impl AddAssign for U256 {
	fn add_assign(
		&mut self,
		other: U256
	) {
		*self = self.overflowing_add(other).0;
	}
}

impl Sub for U256 {
	type Output = U256;

	fn sub(
		self,
		other: U256
	) -> U256 {
		self.overflowing_sub(other).0
	}
}

impl SubAssign for U256 {
	fn sub_assign(
		&mut self,
		other: U256
	) {
		*self = self.overflowing_sub(other).0;
	}
}

#[test]
fn test_overflowing_add_and_sub() {
	let one = U256::one();
	let zero = U256::zero();

	// 1 + 1 = 2
	let mut result = one.overflowing_add(one.clone()).0;
	assert!(result.words[0] == 0);
	assert!(result.words[1] == 0);
	assert!(result.words[2] == 0);
	assert!(result.words[3] == 2);

	// 2 - 1 = 1
	result = result.overflowing_sub(one).0;
	assert!(result.words[0] == 0);
	assert!(result.words[1] == 0);
	assert!(result.words[2] == 0);
	assert!(result.words[3] == 1);

	// 0 - 1 = MAX
	result = zero.overflowing_sub(one).0;
	assert!(result.words[0] == u64::MAX);
	assert!(result.words[1] == u64::MAX);
	assert!(result.words[2] == u64::MAX);
	assert!(result.words[3] == u64::MAX);

	// MAX + 1 = 0
	result = U256::MAX.overflowing_add(one).0;
	assert!(result.words[0] == 0);
	assert!(result.words[1] == 0);
	assert!(result.words[2] == 0);
	assert!(result.words[3] == 0);

	assert!(one + zero == one);
	assert!(zero - one == U256::MAX);
	assert!(U256::MAX + one == zero);
}
