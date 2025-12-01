/**
 * Linked List
 * 1. definition
 * 2. traverse a list
 *
 * prev.next                    // Type: Option<Box<ListNode>>
 * prev.next.as_mut()           // Type: Option<&mut Box<ListNode>>
 * prev.next.as_mut().unwrap()  // Type: &mut Box<ListNode>
 */

// definition
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;
impl Solution {
    pub fn immutable_traverse(head: Option<Box<ListNode>>) {
        println!("immutable: ");
        let mut curr = &head; // immutable borrow

        while let Some(node) = curr {
            // node &Box<T>
            println!("{:?}", node.val);
            curr = &node.next;
        }
        println!("done");
    }

    pub fn mutable_traverse(mut head: Option<Box<ListNode>>) {
        // add 1 to each node'val
        println!("mutable: ");
        let mut curr = &mut head; // mutable borrow

        // while let Some(ref mut node) = curr {
        //     node.val += 1;
        //     println!("{:?}", node.val);

        //     curr = &mut node.next;
        // }

        // while curr.is_some() {
        //     println!("{:?}", curr.as_ref().unwrap().val);
        //     curr = &mut curr.as_mut().unwrap().next;
        // }

        println!("done");
    }
}
fn main() {
    let ll: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        })),
    }));
    // 1-> 2-> 3-> 4 -> 5-> 5
    // Solution::immutable_traverse(ll);

    Solution::mutable_traverse(ll);
}
