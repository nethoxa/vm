use std::ops::Not;

use super::U256;

impl Not for U256 {
	type Output = U256;

	fn not(self) -> U256 {
		U256 {
			words: [
				!self.words[0],
				!self.words[1],
				!self.words[2],
				!self.words[3]
			]
		}
	}
}

#[test]
fn test_not() {
	let zero = U256::zero();
	let num = U256::from(10u64);
	let not = !num;

	assert!(zero == num & not);
}
