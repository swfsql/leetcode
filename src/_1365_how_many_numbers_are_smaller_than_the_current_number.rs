//! # How Many Numbers Are Smaller Than the Current Number
//! https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/
//!
pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        use std::convert::TryInto;
        _smaller_numbers_than_current(nums.into_iter().map(|n| n.try_into().unwrap()).collect())
            .map(Into::into)
            .collect()
    }
}

// 0 <= nums[i] <= 100
// 2 <= nums.len() <= 500
pub fn _smaller_numbers_than_current(nums: Vec<u8>) -> impl Iterator<Item = u16> {
    let clone = nums.clone();
    nums.into_iter()
        .map(move |n| clone.iter().filter(|m| **m < n).count() as u16)
}
