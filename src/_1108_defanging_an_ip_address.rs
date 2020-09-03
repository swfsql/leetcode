//! # Defanging an IP Address
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1108_defanging_an_ip_address.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1108_defanging_an_ip_address.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1108_defanging_an_ip_address/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/defanging-an-ip-address/

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
