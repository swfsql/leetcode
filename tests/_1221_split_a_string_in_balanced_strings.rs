use leetcode::_1221_split_a_string_in_balanced_strings::*;

#[test]
fn _1221_split_a_string_in_balanced_strings_0() {
    assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".into()), 4);
}

#[test]
fn _1221_split_a_string_in_balanced_strings_1() {
    assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".into()), 3);
}

#[test]
fn _1221_split_a_string_in_balanced_strings_2() {
    assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".into()), 2);
}
