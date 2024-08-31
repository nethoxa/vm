use std::ops::{BitAnd, BitAndAssign};

use super::U256;

impl BitAnd for U256 {
    type Output = U256;
    
    fn bitand(self, other: U256) -> U256 {
        let mut result = [0; 4];

        for i in 0..4 {
            result[3 - i] = self.words[3 - i] & other.words[3 - i];
        }

        U256::new(result)
    }
}

impl BitAndAssign for U256 {
    fn bitand_assign(&mut self, other: U256) {
        for i in 0..4 {
            self.words[3 - i] &= other.words[3 - i]
        }
    }
}

#[test]
fn test_and() {
    let zero = U256::zero();
    let one = U256::one();
    let max_sub_one = U256::MAX - one;

    assert!(zero & zero == zero);
    assert!(zero & one == zero);
    assert!(one & one == one);
    assert!(max_sub_one & one == zero);
}