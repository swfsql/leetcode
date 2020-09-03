//! # Sum of Nodes with Even-Valued Grandparent
//!
//! [![src]][srcurl] [![Tests]][testsurl] [![Doc]][docurl] [![Problem]][problemurl]
//!
//! [src]: https://img.shields.io/badge/source-swfsql/leetcode-8DA0CB?style=for-the-badge&labelColor=555555&logo=github
//! [srcurl]: https://github.com/swfsql/leetcode/blob/master/src/_1315_sum_of_nodes_with_even_valued_grandparent.rs
//! [Tests]: https://img.shields.io/badge/tests-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [testsurl]: https://github.com/swfsql/leetcode/blob/master/tests/_1315_sum_of_nodes_with_even_valued_grandparent.rs
//! [doc]: https://img.shields.io/badge/docs-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [docurl]: https://swfsql.github.io/leetcode/doc/leetcode/_1315_sum_of_nodes_with_even_valued_grandparent/index.html
//! [Problem]: https://img.shields.io/badge/problem-555555?style=for-the-badge&labelColor=555555&logoColor=white
//! [problemurl]: https://leetcode.com/problems/sum-of-nodes-with-even-valued-grandparent/

pub struct Solution;
pub use crate::common::simple_tree_node::{convert_children, SimpleTreeNode, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::convert::TryInto;
        let root: Option<SimpleTreeNode> = root.as_ref().map(convert_children).map(|b| *b);
        _sum_even_grandparent(root).try_into().unwrap()
    }
}

/// Idiomatic entry point.
pub fn _sum_even_grandparent(root: Option<SimpleTreeNode>) -> u32 {
    root.map_or_else(|| 0, |n| n.sum_even_grandparent(false, false))
}

impl SimpleTreeNode {
    pub fn sum_even_grandparent(&self, dad_even: bool, granpa_even: bool) -> u32 {
        use std::convert::TryFrom;
        self.children()
            // for each child
            .iter()
            // get it's sum
            .map(|c| {
                c.map_or_else(
                    || 0,
                    |_c| _c.sum_even_grandparent(self.val % 2 == 0, dad_even),
                )
            })
            // also adds self value
            .chain(
                [if granpa_even {
                    u32::try_from(self.val).unwrap()
                } else {
                    0
                }; 1]
                    .iter()
                    .cloned(),
            )
            .sum()
    }
}
