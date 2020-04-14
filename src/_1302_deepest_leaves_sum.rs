//! # Deepest Leaves Sum
//! https://leetcode.com/problems/deepest-leaves-sum/
//!
pub struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct SimpleTreeNode {
    pub val: i32,
    pub left: Option<Box<SimpleTreeNode>>,
    pub right: Option<Box<SimpleTreeNode>>,
}

fn convert_children(child: &Rc<RefCell<TreeNode>>) -> Box<SimpleTreeNode> {
    Box::new((&*(*child).borrow()).into())
}

impl From<&TreeNode> for SimpleTreeNode {
    fn from(node: &TreeNode) -> Self {
        Self {
            val: node.val,
            left: node.left.as_ref().map(convert_children),
            right: node.right.as_ref().map(convert_children),
        }
    }
}

impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root: Option<SimpleTreeNode> = root.as_ref().map(convert_children).map(|b| *b);
        _deepest_leaves_sum(root)
    }
}

/// Idiomatic entry point.
pub fn _deepest_leaves_sum(root: Option<SimpleTreeNode>) -> i32 {
    root.map(|n| n.deepest_leaves_sum(0).1).unwrap_or_else(|| 0)
}

impl SimpleTreeNode {
    pub fn children(&self) -> [Option<&SimpleTreeNode>; 2] {
        [
            self.left.as_ref().map(|c| c.as_ref()),
            self.right.as_ref().map(|c| c.as_ref()),
        ]
    }
    pub fn deepest_leaves_sum(&self, last_depth: usize) -> (usize, i32) {
        self.children()
            .iter()
            .map(|c| {
                c.map(|_c| _c.deepest_leaves_sum(last_depth + 1))
                    // children inexistent
                    .unwrap_or_else(|| (0, 0))
            })
            // also adds self value (used in case this is a leaf)
            .chain([(last_depth, self.val); 1].iter().cloned())
            // get the best result for this node
            .fold(
                (0, 0),
                |(last_max_depth, last_max_sum), (this_depth, this_max_sum)| {
                    let res = if this_depth == last_max_depth {
                        (last_max_depth, last_max_sum + this_max_sum)
                    } else if this_depth > last_max_depth {
                        (this_depth, this_max_sum)
                    } else {
                        (last_max_depth, last_max_sum)
                    };
                    res
                },
            )
    }
}
