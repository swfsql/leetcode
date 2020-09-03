//! # Jewels and Stones
//!
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_0771_jewels_and_stones.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_0771_jewels_and_stones.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_0771_jewels_and_stones/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/jewels-and-stones/

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
