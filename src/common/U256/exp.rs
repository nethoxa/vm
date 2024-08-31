use crate::common::errors::MathError;

use super::U256;

impl U256 {
	fn overflowing_pow(
		self,
		exp: U256
	) -> (U256, bool) {
		todo!()
	}

	fn checked_pow(
		self,
		exp: U256
	) -> Result<U256, MathError> {
		todo!()
	}
}
