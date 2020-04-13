use leetcode::_1313_decompress_run_length_encoded_list::*;

#[test]
fn _1313_decompress_run_length_encoded_list_0() {
    assert_eq!(
        Solution::decompress_rl_elist(vec![1, 2, 3, 4]),
        vec![2, 4, 4, 4]
    );
}
