use leetcode::_1108_defanging_an_ip_address::*;

#[test]
fn _1108_defanging_an_ip_address_0() {
    assert_eq!(Solution::defang_i_paddr("1.1.1.1".into()), "1[.]1[.]1[.]1");
}
