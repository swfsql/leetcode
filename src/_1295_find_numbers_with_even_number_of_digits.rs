//! # Find Numbers with Even Number of Digits
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1295_find_numbers_with_even_number_of_digits.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1295_find_numbers_with_even_number_of_digits.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1295_find_numbers_with_even_number_of_digits/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/find-numbers-with-even-number-of-digits/

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
