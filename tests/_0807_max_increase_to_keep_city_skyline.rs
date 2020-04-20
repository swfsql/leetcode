use leetcode::_0807_max_increase_to_keep_city_skyline::*;

#[test]
fn _0807_max_increase_to_keep_city_skyline_0() {
    assert_eq!(
        Solution::max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0]
        ]),
        35
    );
}
