/**
 * 203. Remove Linked List Elements
 *
 * Given the head of linked list, remove node.val == val
 *
 * return the new head
 */

#[derive(PartialEq, Eq, Debug, Clone)]
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut prev = &mut dummy;

        while prev.next.is_some() {
            if prev.next.as_ref().unwrap().val == val {
                prev.next = prev.next.as_mut().unwrap().next.take();
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

fn main() {
    // 1 -> 10 -> 4 -> 3 -> 5 -> 3
    let ll: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 10,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            })),
        })),
    }));

    let result = Solution::remove_elements(ll, 3); // 1 -> 10 -> 4 -> 5

    println!("{:?}", result);
}
