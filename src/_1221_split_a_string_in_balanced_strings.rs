//! # Split a String in Balanced Strings
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1221_split_a_string_in_balanced_strings.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1221_split_a_string_in_balanced_strings.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1221_split_a_string_in_balanced_strings/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/split-a-string-in-balanced-strings/

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
