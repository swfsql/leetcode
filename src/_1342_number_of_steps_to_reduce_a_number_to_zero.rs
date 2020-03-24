//! # Number of Steps to Reduce a Number to Zero
//! https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
//!
pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        use std::convert::TryInto;
        _number_of_steps(num.try_into().unwrap())
            .try_into()
            .unwrap()
    }
}

pub fn _number_of_steps(n: u32) -> u32 {
    n.count_ones() * 2 + n.count_zeros() - n.leading_zeros() - 1
}
