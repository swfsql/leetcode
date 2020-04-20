//! # Queries on a Permutation With Key
//! https://leetcode.com/problems/queries-on-a-permutation-with-key/
//!
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
