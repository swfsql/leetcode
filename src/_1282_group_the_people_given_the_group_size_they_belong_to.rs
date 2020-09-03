//! # Group the People Given the Group Size They Belong To
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1282_group_the_people_given_the_group_size_they_belong_to.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1282_group_the_people_given_the_group_size_they_belong_to.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1282_group_the_people_given_the_group_size_they_belong_to/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
//!
//! ## Constraints
//!
//! - groupSizes.length == n
//! - 1 <= n <= 500
//! - 1 <= groupSizes[i] <= n
//!

pub struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        use std::convert::TryFrom;
        _group_the_people(
            group_sizes
                .into_iter()
                .map(|group_size| u16::try_from(group_size).unwrap()),
        )
        .map(|g| {
            g.into_iter()
                .map(|id| i32::try_from(id).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
    }
}

/// Idiomatic entry point.
pub fn _group_the_people(
    group_sizes: impl Iterator<Item = u16>,
) -> impl Iterator<Item = Vec<usize>> {
    use std::collections::HashMap;
    let groups = group_sizes
        .enumerate()
        .fold(HashMap::new(), |mut acc, (id, group_size)| {
            acc.entry(group_size as usize)
                .or_insert(Vec::new())
                .push(id);
            acc
        });
    groups
        .into_iter()
        .flat_map(|(group_size, ids): (usize, Vec<usize>)| {
            ids.chunks_exact(group_size)
                .map(|_ids| _ids.to_vec())
                .collect::<Vec<Vec<_>>>()
        })
}
