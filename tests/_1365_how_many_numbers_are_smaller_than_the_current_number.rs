use leetcode::_1365_how_many_numbers_are_smaller_than_the_current_number::*;

#[test]
fn _1365_how_many_numbers_are_smaller_than_the_current_number_0() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
}
