use leetcode::_1282_group_the_people_given_the_group_size_they_belong_to::*;

#[test]
fn _1282_group_the_people_given_the_group_size_they_belong_to_0() {
    assert_eq!(
        Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec![vec![5], vec![0, 1, 2], vec![3, 4, 6]]
    );
}
