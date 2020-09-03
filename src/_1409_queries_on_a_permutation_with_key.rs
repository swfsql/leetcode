//! # Queries on a Permutation With Key
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1409_queries_on_a_permutation_with_key.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1409_queries_on_a_permutation_with_key.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1409_queries_on_a_permutation_with_key/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/queries-on-a-permutation-with-key/

pub struct Solution;

impl Solution {
    pub fn process_queries(queries: Vec<i32>, m: i32) -> Vec<i32> {
        use std::convert::TryInto;
        _process_queries(
            queries.into_iter().map(|q| q.try_into().unwrap()),
            m.try_into().unwrap(),
        )
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }
}

/// Idiomatic entry point.
pub fn _process_queries(queries: impl Iterator<Item = u16>, m: u16) -> impl Iterator<Item = usize> {
    let mut p: Vec<_> = (1..=m).collect();
    queries.map(move |q| {
        let pos = p.iter().position(|pi| *pi == q).unwrap();
        p.remove(pos);
        p.insert(0, q);
        pos
    })
}
