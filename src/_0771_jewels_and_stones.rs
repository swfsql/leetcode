//! # Jewels and Stones
//! https://leetcode.com/problems/jewels-and-stones/
//!
pub struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        use std::convert::TryInto;
        _num_jewels_in_stones(j.chars(), s.chars())
            .try_into()
            .unwrap()
    }
}

/// Idiomatic entry point.
pub fn _num_jewels_in_stones(
    j: impl Iterator<Item = char>,
    s: impl Iterator<Item = char>,
) -> usize {
    let j: std::collections::HashSet<_> = j.collect();
    s.filter(|item| j.contains(item)).count()
}
