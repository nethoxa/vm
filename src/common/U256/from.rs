use super::U256;

impl From<u16> for U256 {
	fn from(value: u16) -> U256 { U256::new([0, 0, 0, value as u64]) }
}

impl From<u32> for U256 {
	fn from(value: u32) -> U256 { U256::new([0, 0, 0, value as u64]) }
}

impl From<usize> for U256 {
	fn from(value: usize) -> U256 { U256::new([0, 0, 0, value as u64]) }
}

impl From<u64> for U256 {
	fn from(value: u64) -> U256 { U256::new([0, 0, 0, value]) }
}

impl From<u128> for U256 {
	fn from(value: u128) -> U256 { U256::new([0, 0, (value >> 64) as u64, value as u64]) }
}

#[test]
fn test_stuff() {
	assert!(u128::from(U256::from(6u128)) == 6u128);
	assert!(u128::from(U256::from((u64::MAX as u128) + 5)) == (u64::MAX as u128) + 5);
}
