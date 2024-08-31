use super::U256;

impl From<u16> for U256 {
	fn from(value: u16) -> U256 { todo!() }
}

impl From<u32> for U256 {
	fn from(value: u32) -> U256 { todo!() }
}

impl From<usize> for U256 {
	fn from(value: usize) -> U256 { todo!() }
}

impl From<u64> for U256 {
	fn from(value: u64) -> U256 { U256::new([0, 0, 0, value]) }
}

impl From<u128> for U256 {
	fn from(value: u128) -> U256 { todo!() }
}
