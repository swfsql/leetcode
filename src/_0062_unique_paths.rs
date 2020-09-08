//! # Unique Paths
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_0062_unique_paths.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_0062_unique_paths.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_0062_unique_paths/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/unique-paths/

pub struct Solution;

use std::convert::TryInto;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        Self::_unique_paths(m.try_into().unwrap(), n.try_into().unwrap())
            .try_into()
            .unwrap()
    }

    /// Idiomatic entry point.
    ///
    /// ## Rationale
    ///
    /// In principle, we replicate the grid, and also replace
    /// the Finish with the Start, and go from the New Finish
    /// towards the new Start.  
    /// We walk on this new grid left->right, top->bottom.  
    /// Each item holds a single value, where that value is the
    /// final result as if it were the last stopping point item
    /// in the grid (the new Start). So each value contains the
    /// "sum of it's ancestors", and in reality they will be
    /// used by their "sucessors".  
    /// One value's ancestors are simply the one on their left,
    /// and the one on their top.
    ///
    /// ## Implementation
    ///
    /// In the code, a grid is not created, but a single line is,
    /// having one item for each row (`m`). Each "top ancestor"
    /// is simply replaced on each iteration, becoming the next
    /// "top ancestor". The next "top ancestor" also accumulates
    /// from the "next left ancestor", which is cached as `prev`.
    ///
    pub fn _unique_paths(m: u8, n: u8) -> u32 {
        // vec mapping the columns
        let mut v = vec![1u32; m as usize];
        // for each row..
        for _ in 1..n {
            // `prev` is the "next left ancestor"
            let mut prev: u32 = 0;
            // for each column..
            for top_ancestor in v.iter_mut() {
                // prepares the "next top ancestor"
                // (which will also be the "next left ancestor"
                // for the next column)
                prev = prev + *top_ancestor;
                // replaces the "next top ancestor"
                // into the "last top ancestor"
                // (for this column)
                *top_ancestor = prev;
            }
        }
        // safe unwrap since `n` and `m` > `0`.
        *v.last().unwrap()
    }
}
