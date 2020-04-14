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
    let digits = binary_to_bcd(n).map(i32::from);
    digits.clone().product::<i32>() - digits.sum::<i32>()
}

/// https://my.eng.utah.edu/~nmcdonal/Tutorials/BCDTutorial/BCDConversion.html
pub fn binary_to_bcd(n: u32) -> impl Clone + Iterator<Item = u8> {
    // u32 max = 4_294_967_295 = 10 digits
    // bcd requires 4 bits for each digit,
    // so the result requires 40 bits in total
    //
    // if the input is inlined (32 bits),
    // the left shift could make space for freeing the input
    //
    // but for simplicity, using a u128 is enought space
    // for both the result (40 bits) and the input (32 bits)
    // (128 > 72 bits)
    //

    let offset = 32;
    let mut conv: u128 = n.into();
    // 32 left shifts
    for _ in 0..offset {
        // step 1: verify all decimal columns; if >= 5, sum with 3
        for col in 0..10 {
            let shift = offset + 4 * col;
            let window = (conv >> shift) & 0xF;
            if window >= 5 {
                conv += 3 << shift;
            }
        }
        // step 2: left rotation
        conv = conv.rotate_left(1);
    }

    // erases the input
    conv >>= offset;
    (0..10)
        .map(move |c| (conv >> c * 4 & 0xF) as u8)
        // skip trailing zeroes
        .rev()
        .skip_while(|d| *d == 0)
}
