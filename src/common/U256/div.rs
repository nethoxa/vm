use std::ops::{Div, DivAssign, Rem, RemAssign};

use crate::common::errors::MathError;

use super::U256;

impl U256 {
	fn div_rem_long(self, other: U256) -> Result<(U256, U256), MathError> {
		if other == U256::zero() {
			return Err(MathError::DivisionByZero)
		}

		if self < other {
			return Ok((U256::zero(), self))
		}

		if other == U256::one() {
			return Ok((self, U256::zero()))
		}

		let mut quotient = U256::zero();
		let mut remainder = U256::zero();

		for i in 0..256 {
            // Left shift remainder by 1 bit
            remainder = remainder << U256::one();

            // Set the least significant bit of remainder to the i-th bit of dividend
            if self.bit(255 - i as u64).unwrap() {
                remainder.set_bit(0, true).unwrap();
            }

            // If remainder >= divisor, subtract divisor from remainder and set quotient bit
            if remainder >= other {
                remainder = remainder - other;
                quotient.set_bit(255 - i as u64, true).unwrap();
            }
        }

		Ok((quotient, remainder))
	}
}

impl Div for U256 {
	type Output = U256;

	fn div(
		self,
		other: U256
	) -> U256 {
		self.div_rem_long(other).unwrap().0
	}
}

impl DivAssign for U256 {
	fn div_assign(
		&mut self,
		other: U256
	) {
		*self = self.div_rem_long(other).unwrap().0;
	}
}

impl Rem for U256 {
	type Output = U256;

	fn rem(
		self,
		other: U256
	) -> U256 {
		self.div_rem_long(other).unwrap().1
	}
}

impl RemAssign for U256 {
	fn rem_assign(
		&mut self,
		other: U256
	) {
		*self = self.div_rem_long(other).unwrap().1;
	}
}

#[test]
fn test_div_rem() {
	let ten = U256::from(10u64);
	let two = U256::from(2u64);
	let three = U256::from(3u64);
	let zero = U256::zero();

	assert!(ten / two == U256::from(5u64));
	assert!(ten / three == three);
	assert!(ten % two == zero);
	assert!(ten % three == U256::one());
}

#[test]
fn test_panics_div() {
	let ten = U256::from(10u64);
	let zero = U256::zero();

	let _ = ten / zero;
}

#[test]
fn test_panics_rem() {
	let ten = U256::from(10u64);
	let zero = U256::zero();

	let _ = ten % zero;
}