use leetcode::_0010_regular_expression_matching::*;

#[test]
fn _0010_regular_expression_matching_0() {
    assert_eq!(Solution::_is_match("aa", "a"), false);
    assert_eq!(Solution::_is_match("aa", "a*"), true);
    assert_eq!(Solution::_is_match("ab", ".*"), true);
    assert_eq!(Solution::_is_match("aab", "c*a*b"), true);
    assert_eq!(Solution::_is_match("mississippi", "mis*is*p*."), false);
    assert_eq!(Solution::_is_match("mississippi", "mis*is*ip*."), true);
}

// tests the dynamic programming requirement
// (time-up BADEND)
#[test]
fn _0010_regular_expression_matching_1() {
    assert_eq!(
        Solution::_is_match(
            "bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab",
            "b.*b.*ab.*.*ba.*b.*.*b.*.*.*bba"
        ),
        false
    );
}

// tests the affine logic
#[test]
fn _0010_regular_expression_matching_2() {
    assert_eq!(
        Solution::_is_match("mississippi", "mis?s?iss?ip?p?p?."),
        true
    );

    // can consume one input
    // ie. ignore one input while ignoring itself
    assert_eq!(Solution::_is_match("axb", "ax?b"), true);
    // like ?
    assert_eq!(Solution::_is_match("axxb", "ax?x?b"), true);
    // can ignore one string input
    assert_eq!(Solution::_is_match("ab", "ax?b"), true);
    assert_eq!(Solution::_is_match("ab", "ax?x?x?x?b"), true);
    assert_eq!(Solution::_is_match("", "x?"), true);
    assert_eq!(Solution::_is_match("", "x?x?"), true);
    // cannot ignore more than 1 input
    assert_eq!(Solution::_is_match("axxb", "ax?b"), false);
    assert_eq!(Solution::_is_match("ab", "x?"), false);
    assert_eq!(Solution::_is_match("abc", "x?x?"), false);
}

// tests the relevant logic
#[test]
fn _0010_regular_expression_matching_3() {
    assert_eq!(Solution::_is_match("mississippi", "mis+is+ip+."), true);

    // must consume one input
    // ignoring input while ignoring and/or consuming itself
    assert_eq!(Solution::_is_match("axb", "ax+b"), true);
    assert_eq!(Solution::_is_match("a", "a+"), true);
    assert_eq!(Solution::_is_match("aa", "a+a+"), true);
    // may fail to consume the minimum one input
    assert_eq!(Solution::_is_match("ab", ".+ab"), false);
    assert_eq!(Solution::_is_match("ab", "a.+b"), false);
    assert_eq!(Solution::_is_match("ab", "ab.+"), false);
    assert_eq!(Solution::_is_match("", ".+"), false);
    assert_eq!(Solution::_is_match("a", ".+.+"), false);
    // after the first consumption,
    // behaves like *
    assert_eq!(Solution::_is_match("axxxxxb", "ax+b"), true);
    assert_eq!(Solution::_is_match("axxxxxb", ".+"), true);
}
