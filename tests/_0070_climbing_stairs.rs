use leetcode::_0070_climbing_stairs::*;

#[test]
fn _0070_climbing_stairs_0() {
    assert_eq!(Solution::_climb_stairs(2), 2);
    assert_eq!(Solution::_climb_stairs(3), 3);
    //
    assert_eq!(Solution::_climb_stairs(1), 1);
}
