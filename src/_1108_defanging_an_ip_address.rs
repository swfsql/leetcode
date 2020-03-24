//! # Defanging an IP Address
//! https://leetcode.com/problems/defanging-an-ip-address/
//!
pub struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        use std::str::FromStr;
        defang_i_paddr(FromStr::from_str(&address).unwrap())
    }
}

pub fn defang_i_paddr(addr: std::net::Ipv4Addr) -> String {
    addr.to_string().replace('.', "[.]")
}
