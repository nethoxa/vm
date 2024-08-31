use std::ops::{Mul, MulAssign};

use super::U256;

impl U256 {
	fn overflowing_mul(self, other: U256) -> (U256, bool) {
		let mut result = U256::zero();
        let mut overflow = false;

        for i in 0..4 {
            let mut carry = 0u64;
            for j in 0..4 {
                let k = i + j;
                if k >= 4 {
                    overflow |= self.words[i] != 0 && other.words[j] != 0;
                } else {
                    let (hi, lo) = mul_carry_shift(self.words[i], other.words[j], carry, result.words[k]);
                    result.words[k] = lo;
                    carry = hi;
                }
            }
            if carry > 0 {
                overflow = true;
            }
        }

        (result, overflow)
	}

	// Helper function to perform multiplication with carry and shift
	fn mul_carry_shift(a: u64, b: u64, c: u64, d: u64) -> (u64, u64) {
		let (hi, lo) = u128::from(a).overflowing_mul(u128::from(b));
		let (lo, overflow1) = lo.overflowing_add(u128::from(c));
		let (lo, overflow2) = lo.overflowing_add(u128::from(d));
		let hi = hi + u128::from(overflow1) + u128::from(overflow2);
		(hi as u64, lo as u64)
}
}

impl Mul for U256 {
	type Output = U256;

	fn mul(
		self,
		other: U256
	) -> U256 {
		todo!()
	}
}

impl MulAssign for U256 {
	fn mul_assign(
		&mut self,
		other: U256
	) {
		todo!()
	}
}
