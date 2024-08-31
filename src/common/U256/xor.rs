use std::ops::{BitXor, BitXorAssign};

use super::U256;

impl BitXor for U256 {
	type Output = U256;

	fn bitxor(
		self,
		other: U256
	) -> U256 {
		todo!()
	}
}

impl BitXorAssign for U256 {
	fn bitxor_assign(
		&mut self,
		other: U256
	) {
		todo!()
	}
}
