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
    use std::collections::HashMap;
    // sorts
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    // creates a map of a value into it's index position
    // (when it first appeared in the sorted nums)
    let map = sorted.into_iter().enumerate().fold(
        HashMap::<u8, u16>::new(),
        |mut acc, (index, value)| {
            acc.entry(value).or_insert(index as u16);
            acc
        },
    );
    nums.into_iter().map(move |n| map[&n])
}
