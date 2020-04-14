//! # Group the People Given the Group Size They Belong To
//! https://leetcode.com/problems/group-the-people-given-the-group-size-they-belong-to/
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
