use leetcode::_0771_jewels_and_stones::*;

#[test]
fn _0771_jewels_and_stones_0() {
    assert_eq!(
        Solution::num_jewels_in_stones("aA".into(), "aAAbbbb".into()),
        3
    );
}
