use leetcode::_0044_wildcard_matching::*;

#[test]
fn _0044_wildcard_matching_0() {
    assert_eq!(Solution::_is_match("aa", "p"), false);
    assert_eq!(Solution::_is_match("aa", "*"), true);
    assert_eq!(Solution::_is_match("cb", "?a"), false);
    assert_eq!(Solution::_is_match("adceb", "a*b"), true);
    assert_eq!(Solution::_is_match("acdcb", "a*c?b"), false);
    //
    assert_eq!(Solution::_is_match("", ""), true);
    assert_eq!(Solution::_is_match("", "*"), true);
    assert_eq!(Solution::_is_match("", "******"), true);
    assert_eq!(Solution::_is_match("aaaaaaaaa", "*********"), true);
    assert_eq!(Solution::_is_match("aaaaaaaaa", "*a*a*a*a*"), true);
    assert_eq!(Solution::_is_match("", "?"), false);
}
