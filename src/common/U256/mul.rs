use std::ops::{Mul, MulAssign};

use super::U256;

impl U256 {
	pub fn overflowing_mul(
		self,
		other: U256
	) -> (U256, bool) {
		todo!()
	}
}

impl Mul for U256 {
	type Output = U256;

	fn mul(
		self,
		other: U256
	) -> U256 {
		todo!()
	}
}

impl MulAssign for U256 {
	fn mul_assign(
		&mut self,
		other: U256
	) {
		todo!()
	}
}
