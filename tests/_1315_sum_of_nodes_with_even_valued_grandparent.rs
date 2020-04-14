use leetcode::_1315_sum_of_nodes_with_even_valued_grandparent::*;
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
fn _1315_sum_of_nodes_with_even_valued_grandparent_0() {
    let root = node(
        6,
        node(
            7,
            node(2, node(9, None, None), None),
            node(7, node(1, None, None), node(4, None, None)),
        ),
        node(8, node(1, None, None), node(3, None, node(5, None, None))),
    );

    assert_eq!(Solution::sum_even_grandparent(root), 18);
}
