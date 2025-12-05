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

struct Solution;

impl Solution {
    // pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut result: Vec<i32> = vec![];
    //     if root.is_none() {
    //         return result;
    //     }

    //     let mut stack1: Vec<Rc<RefCell<TreeNode>>> = vec![root.unwrap()]; // root -> right -> left
    //     let mut stack2: Vec<Rc<RefCell<TreeNode>>> = vec![]; // left -> right -> root

    //     while let Some(node) = stack1.pop() {
    //         stack2.push(Rc::clone(&node)); // node is Rc<RefCell<TreeNode>>

    //         let node_ref = node.borrow(); // Ref<TreeNode>
    //         node_ref.left
    //         Type: Option<Rc<RefCell<TreeNode>>>
    //         BUT accessed through a borrow (Ref), so it's really:
    //         &Option<Rc<RefCell<TreeNode>>>

    //         if let Some(node) = node_ref.left {
    //                            ^^^^^^^^^^^^^^^
    //                           Trying to MOVE out of a borrowed reference
    //                           ❌ COMPILE ERROR!
    //             stack1.push(Rc::clone(&node));
    //         }

    //         if let Some(node) = node_ref.right {
    //             stack1.push(Rc::clone(&node));
    //         }
    //     }

    //     while let Some(node) = stack2.pop() {
    //         node Rc<T>
    //         let node_ref = node.borrow();
    //         result.push(node_ref.val);
    //     }

    //     result
    // }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {}
}
