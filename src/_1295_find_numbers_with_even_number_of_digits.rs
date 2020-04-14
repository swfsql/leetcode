//! # Find Numbers with Even Number of Digits
//! https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
//!
pub struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        use std::convert::{TryFrom, TryInto};
        _find_numbers(nums.into_iter().map(|n| u32::try_from(n).unwrap()))
            .try_into()
            .unwrap()
    }
}

/// Idiomatic entry point.
pub fn _find_numbers(nums: impl Iterator<Item = u32>) -> usize {
    nums.map(crate::common::binary_to_bcd)
        .filter(|bcd: _| bcd.clone().count() % 2 == 0)
        .count()
}
