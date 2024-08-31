use crate::common::errors::MathError;

use super::U256;

impl U256 {
	fn overflowing_pow(
		self,
		exp: U256
	) -> (U256, bool) {
		let result = [0; 4];
		let overflow = false;

		(U256::new(result), overflow)
	}

	fn checked_pow(
		self,
		exp: U256
	) -> Result<U256, MathError> {
		let result = [0; 4];
		let overflow = false;

		Ok(U256::new(result))
	}
}
