use std::cell::RefCell
use std::rc::Rc

/**
 *
 */
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
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

impl Solution {
    // pub fn inorder_traversal_iteration (root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut result: Vec<i32> = vec![];
    //     let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    //     let mut curr = root;

    //     if curr.is_none() {
    //         return result;
    //     }

    //     while !curr.is_none() || !stack.is_empty() {
    //         while let Some(node) = &curr {
    //             node Rc<T>
    //             stack.push(Rc::clone(node));
    //             curr = Rc::clone(node.borrow().left); // <-- ERROR
    //         }


    //         if let Some(node) = stack.pop() {
    //             let node_ref = node.borrow(); // Type: Ref<TreeNode>
    //             result.push(node_ref.val);

    //             move to the right subtree
    //             curr = Rc::clone(node.borrow().right); // <-- ERROR
    //             node.borrow().right is  &Option<Rc<...>>
    //             Rc::clone wants &Rc<...>, not &Option<Rc<...>>
    //         }
    //     }

    //     result
    // }
}