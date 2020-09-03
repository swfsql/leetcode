//! # Number of Steps to Reduce a Number to Zero
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1342_number_of_steps_to_reduce_a_number_to_zero.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1342_number_of_steps_to_reduce_a_number_to_zero.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1342_number_of_steps_to_reduce_a_number_to_zero/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        use std::convert::TryInto;
        _number_of_steps(num.try_into().unwrap())
            .try_into()
            .unwrap()
    }
}

pub fn _number_of_steps(n: u32) -> u32 {
    n.count_ones() * 2 + n.count_zeros() - n.leading_zeros() - 1
}
