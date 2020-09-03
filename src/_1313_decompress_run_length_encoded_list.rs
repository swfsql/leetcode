//! # Decompress Run-Length Encoded List
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1313_decompress_run_length_encoded_list.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1313_decompress_run_length_encoded_list.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1313_decompress_run_length_encoded_list/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/decompress-run-length-encoded-list/

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
