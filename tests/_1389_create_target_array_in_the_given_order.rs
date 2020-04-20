use leetcode::_1389_create_target_array_in_the_given_order::*;

#[test]
fn _1389_create_target_array_in_the_given_order_0() {
    assert_eq!(
        Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
}
