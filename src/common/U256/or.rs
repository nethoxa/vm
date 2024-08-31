use std::ops::{BitOr, BitOrAssign};

use super::U256;

impl BitOr for U256 {
	type Output = U256;

	fn bitor(
		self,
		other: U256
	) -> U256 {
		todo!()
	}
}

impl BitOrAssign for U256 {
	fn bitor_assign(
		&mut self,
		other: U256
	) {
		todo!()
	}
}
