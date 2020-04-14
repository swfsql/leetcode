//! # Subtract the Product and Sum of Digits of an Integer
//! https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
//!
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
