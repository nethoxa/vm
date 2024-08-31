use std::ops::Neg;

use super::U256;

impl Neg for U256 {
	type Output = U256;

	fn neg(self) -> U256 { U256::zero() - self }
}

#[test]
fn test_neg() {
	let zero = U256::zero();
	let num = U256::from(10u64);
	let negated = -num;

	assert!(zero == negated + num)
}
