use leetcode::_0044_wildcard_matching::*;

#[test]
fn _0044_wildcard_matching_0() {
    assert_eq!(Solution::_is_match("aa", "p"), false);
    assert_eq!(Solution::_is_match("aa", "*"), true);
    assert_eq!(Solution::_is_match("cb", "?a"), false);
    assert_eq!(Solution::_is_match("adceb", "a*b"), true);
    assert_eq!(Solution::_is_match("acdcb", "a*c?b"), false);
    //
    assert_eq!(Solution::_is_match("aaxxxxbc", "aa*?bc"), true);
    assert_eq!(Solution::_is_match("aaxxxxba", "aa*?bc"), false);
    assert_eq!(Solution::_is_match("", ""), true);
    assert_eq!(Solution::_is_match("", "*"), true);
    assert_eq!(Solution::_is_match("", "******"), true);
    assert_eq!(Solution::_is_match("aaaaaaaaa", "*********"), true);
    assert_eq!(Solution::_is_match("aaaaaaaaa", "*a*a*a*a*"), true);
    assert_eq!(Solution::_is_match("", "?"), false);
    assert_eq!(Solution::_is_match("a", ""), false);
}

// tests the dynamic programming requirement
// (time-up BADEND)
#[test]
fn _0044_wildcard_matching_1() {
    assert_eq!(
        Solution::_is_match(
            "bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab",
            "b*b*ab**ba*b**b***bba"
        ),
        false
    );
}

// tests the affine logic
#[test]
fn _0044_wildcard_matching_2() {
    // can consume one input
    // ie. ignore one input while ignoring itself
    assert_eq!(Solution::_is_match("axb", "a!b"), true);
    // like ?
    assert_eq!(Solution::_is_match("axxb", "a!!b"), true);
    // can ignore one string input
    assert_eq!(Solution::_is_match("ab", "a!b"), true);
    assert_eq!(Solution::_is_match("ab", "a!!!!!b"), true);
    assert_eq!(Solution::_is_match("", "!"), true);
    assert_eq!(Solution::_is_match("", "!!"), true);
    // cannot ignore more than 1 input
    assert_eq!(Solution::_is_match("axxb", "a!b"), false);
    assert_eq!(Solution::_is_match("ab", "!"), false);
    assert_eq!(Solution::_is_match("abc", "!!"), false);
}

// tests the relevant logic
#[test]
fn _0044_wildcard_matching_3() {
    // must consume one input
    // ignoring input while ignoring and/or consuming itself
    assert_eq!(Solution::_is_match("axb", "a+b"), true);
    assert_eq!(Solution::_is_match("a", "+"), true);
    assert_eq!(Solution::_is_match("aa", "++"), true);
    // may fail to consume the minimum one input
    assert_eq!(Solution::_is_match("ab", "+ab"), false);
    assert_eq!(Solution::_is_match("ab", "a+b"), false);
    assert_eq!(Solution::_is_match("ab", "ab+"), false);
    assert_eq!(Solution::_is_match("", "+"), false);
    assert_eq!(Solution::_is_match("a", "++"), false);
    // after the first consumption,
    // behaves like *
    assert_eq!(Solution::_is_match("axxxxxb", "a+b"), true);
    assert_eq!(Solution::_is_match("axxxxxb", "+"), true);
}
