//! # Split a String in Balanced Strings
//! https://leetcode.com/problems/split-a-string-in-balanced-strings/
//!
pub struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        use std::convert::TryInto;
        _balanced_string_split(s.chars().map(|c| match c {
            'L' => false,
            'R' => true,
            _ => unreachable!(),
        }))
        .try_into()
        .unwrap()
    }
}

/// Idiomatic entry point.
pub fn _balanced_string_split(s: impl Iterator<Item = bool>) -> usize {
    s.fold(
        (0usize, 0usize, 0usize), // (left, right, counter)
        |acc, c| match match (c, acc) {
            (false, (l, r, sum)) => (
                // got a Left
                l + 1, //
                r,     //
                sum,   //
            ),
            (true, (l, r, sum)) => (
                // got a Right
                l,     //
                r + 1, //
                sum,   //
            ),
        } {
            (l, r, sum) if l == r => (
                0, // (reset is not actually required)
                0, // (reset is not actually required)
                sum + 1,
            ),
            other => other,
        },
    )
    .2 // returns the counter
}
