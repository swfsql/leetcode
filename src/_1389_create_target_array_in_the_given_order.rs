//! # Create Target Array in the Given Order
//! https://leetcode.com/problems/create-target-array-in-the-given-order/
//!
pub struct Solution;

impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        use std::convert::TryInto;
        _create_target_array(
            nums.into_iter().map(|n| n.try_into().unwrap()),
            index.into_iter().map(|i| i.try_into().unwrap()),
        )
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }
}

/// Idiomatic entry point.
pub fn _create_target_array(
    nums: impl Iterator<Item = u8>,
    index: impl Iterator<Item = usize>,
) -> Vec<u8> {
    nums.zip(index).fold(Vec::new(), |mut v, (n, i)| {
        v.insert(i, n);
        v
    })
}
