use leetcode::_0210::*;

#[test]
fn _0210_0() {
    assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
}

#[test]
fn _0210_1() {
    assert_eq!(
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
}

#[test]
fn _0210_2() {
    assert_eq!(
        Solution::find_order(2, vec![vec![0, 1], vec![1, 0]]),
        vec![]
    );
}

#[test]
fn _0210_3() {
    assert_eq!(
        Solution::find_order(4, vec![vec![0, 1], vec![3, 1], vec![1, 3], vec![3, 2]]),
        []
    );
}

#[test]
fn _0210_4() {
    assert_eq!(
        Solution::find_order(
            7,
            vec![
                vec![1, 0],
                vec![0, 3],
                vec![0, 2],
                vec![3, 2],
                vec![2, 5],
                vec![4, 5],
                vec![5, 6],
                vec![2, 4]
            ]
        ),
        vec![6, 5, 4, 2, 3, 0, 1]
    )
}
