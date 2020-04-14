use leetcode::_1281_subtract_the_product_and_sum_of_digits_of_an_integer::*;

#[test]
fn _template_0() {
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
}

#[test]
fn _template_1() {
    assert_eq!(
        Solution::subtract_product_and_sum(162),
        1 * 6 * 2 - (1 + 6 + 2)
    );
}
