use std::cell::RefCell;
use std::rc::Rc;
/**
 * 144. bt preorder traversal
 */

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

struct Solution;

// 1. iteration
// 2. recursion
impl Solution {
    pub fn preorder_traversal_iteration(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        // Early return if root is None
        if root.is_none() {
            return result;
        }

        // Stack holds Rc<RefCell<TreeNode>>, not i32
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap()];

        // Check if stack is empty, not is_some()
        while !stack.is_empty() {
            let curr_node = stack.pop().unwrap();

            // Borrow the node to access its fields
            let node_ref = curr_node.borrow();
            result.push(node_ref.val);

            // Push right first (so left is processed first - stack is LIFO)
            if let Some(right) = &node_ref.right {
                stack.push(Rc::clone(right));
            }

            // Push left second
            if let Some(left) = &node_ref.left {
                stack.push(Rc::clone(left));
            }
        }

        result
    }

    // pub fn preorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {}
}
