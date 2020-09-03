//! # Create Target Array in the Given Order
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1389_create_target_array_in_the_given_order.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1389_create_target_array_in_the_given_order.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1389_create_target_array_in_the_given_order/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/create-target-array-in-the-given-order/

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
