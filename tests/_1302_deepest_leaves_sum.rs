use leetcode::_1302_deepest_leaves_sum::*;
use std::cell::RefCell;
use std::rc::Rc;

fn node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

#[test]
fn _1302_deepest_leaves_sum_0() {
    let root = node(
        1,
        node(2, node(4, node(7, None, None), None), node(5, None, None)),
        node(3, None, node(6, None, node(8, None, None))),
    );
    assert_eq!(Solution::deepest_leaves_sum(root), 15);
}
