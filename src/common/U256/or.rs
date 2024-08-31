use std::ops::{BitOr, BitOrAssign};

use super::U256;

impl BitOr for U256 {
	type Output = U256;

	fn bitor(
		self,
		other: U256
	) -> U256 {
		let mut result = [0; 4];

		for i in 0..4 {
			result[3 - i] = self.words[3 - i] | other.words[3 - i];
		}

		U256::new(result)
	}
}

impl BitOrAssign for U256 {
	fn bitor_assign(
		&mut self,
		other: U256
	) {
		for i in 0..4 {
			self.words[3 - i] = self.words[3 - i] | other.words[3 - i];
		}
	}
}
