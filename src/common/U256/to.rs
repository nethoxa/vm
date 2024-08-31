use super::U256;

impl From<U256> for u8 {
	fn from(value: U256) -> u8 { value.words[3] as u8 }
}

impl From<U256> for u16 {
	fn from(value: U256) -> u16 { value.words[3] as u16 }
}

impl From<U256> for u32 {
	fn from(value: U256) -> u32 { value.words[3] as u32 }
}

impl From<U256> for u64 {
	fn from(value: U256) -> u64 { value.words[3] as u64 }
}

impl From<U256> for usize {
	fn from(value: U256) -> usize { value.words[3] as usize }
}

impl From<U256> for u128 {
	fn from(value: U256) -> u128 { ((value.words[2] as u128) << 64) + value.words[3] as u128 }
}
