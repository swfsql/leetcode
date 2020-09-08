use leetcode::_0062_unique_paths::*;

#[test]
fn _0062_unique_paths_0() {
    assert_eq!(Solution::unique_paths(3, 2), 3);
    assert_eq!(Solution::unique_paths(7, 3), 28);
    //
    assert_eq!(Solution::unique_paths(1, 1), 1);
    assert_eq!(Solution::unique_paths(2, 1), 1);
    assert_eq!(Solution::unique_paths(1, 2), 1);
}
