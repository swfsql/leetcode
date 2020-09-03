//! # Subtract the Product and Sum of Digits of an Integer
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1281_subtract_the_product_and_sum_of_digits_of_an_integer.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1281_subtract_the_product_and_sum_of_digits_of_an_integer.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1281_subtract_the_product_and_sum_of_digits_of_an_integer/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/

pub struct Solution;

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        use std::convert::TryInto;
        _subtract_product_and_sum(n.try_into().unwrap())
    }
}

/// Idiomatic entry point.
pub fn _subtract_product_and_sum(n: u32) -> i32 {
    // since n <= 100_000,
    // max product is 9^5 = 59_049,
    // which fits in an i32::MAX
    let digits = crate::common::binary_to_bcd(n).map(i32::from);
    digits.clone().product::<i32>() - digits.sum::<i32>()
}
