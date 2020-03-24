//! # Course Schedule II
//! https://leetcode.com/problems/course-schedule-ii/
//!
pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    /// Entry point, and interface to avoid using i32 and
    /// Vec<Vec<_>>.
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        use std::convert::{TryFrom, TryInto};
        Self::_find_order(
            num_courses as usize,
            prerequisites.iter().map(|item| {
                (
                    usize::try_from(item[0]).unwrap(),
                    usize::try_from(item[1]).unwrap(),
                )
            }),
        )
        .into_iter()
        .flatten()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()
        .unwrap()
    }

    /// idiomatic entry point.
    pub fn _find_order(n: usize, reqs: impl Iterator<Item = (usize, usize)>) -> Option<Vec<usize>> {
        // mapping of usize into it's requirements/depedency
        let reqs = reqs.fold(HashMap::new(), |mut acc, (item, req)| {
            acc.entry(item).or_insert(Vec::new()).push(req);
            acc
        });

        let mut res = Vec::with_capacity(n);
        let mut added = HashSet::<usize>::new();
        // for a depedency tree, no children may depend on it's
        // parent/grandparent/etc..
        let mut forbidden = HashSet::new();
        for i in 0..n {
            Self::add_depedency(i, &mut added, &reqs, &mut forbidden, &mut res)?;
        }
        Some(res)
    }

    /// Add element n as an depedency
    pub fn add_depedency<'r>(
        n: usize,
        added: &mut HashSet<usize>,
        reqs: &HashMap<usize, Vec<usize>>,
        forbidden: &mut HashSet<usize>,
        res: &'r mut Vec<usize>,
    ) -> Option<&'r mut Vec<usize>> {
        // early return if is a forbidden value
        if forbidden.contains(&n) {
            return None;
        }
        // early return if already inserted
        else if !added.insert(n) {
            return Some(res);
        };

        forbidden.insert(n);
        for i in reqs.get(&n).into_iter().flatten() {
            Self::add_depedency(*i, added, reqs, forbidden, res)?;
        }
        forbidden.remove(&n);

        res.push(n);
        Some(res)
    }
}
