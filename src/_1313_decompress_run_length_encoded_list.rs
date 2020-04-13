//! # Decompress Run-Length Encoded List
//! https://leetcode.com/problems/decompress-run-length-encoded-list/
//!
pub struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        use std::convert::TryFrom;
        let nums = nums.chunks(2).map(|vals| match vals {
            &[freq, val] => (usize::try_from(freq).unwrap(), u8::try_from(val).unwrap()),
            _ => panic!(),
        });
        _decompress_rl_elist(nums)
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
    }
}

/// Idiomatic entry point.
pub fn _decompress_rl_elist(nums: impl Iterator<Item = (usize, u8)>) -> impl Iterator<Item = u8> {
    nums.flat_map(|(freq, val)| std::iter::repeat(val).take(freq))
}
