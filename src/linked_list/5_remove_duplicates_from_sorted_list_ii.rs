/**
 * 82. remove duplicates from sorted list ii
 *
 * Given the head of a sorted linked list,
 * delete all nodes that have duplicate numbers,
 * leaving only distinct numbers from the original list. Return the linked list sorted as well.
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode {
            val: -1,
            next: head,
        });
        let mut prev = &mut dummy;

        while prev.next.is_some() {
            let mut curr = prev.next.as_mut().unwrap();
            if curr.next.is_some() && curr.val == curr.next.as_ref().unwrap().val {
                let dup_val = curr.val;

                while prev.next.is_some() && prev.next.as_ref().unwrap().val == dup_val {
                    prev.next = prev.next.as_mut().unwrap().next.take();
                }
            } else {
                prev = prev.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

fn main() {
    // 1 -> 2 -> 3 -> 3 -> 5 -> 5
    let ll: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 5,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        })),
    }));

    let result = Solution::delete_duplicates(ll);

    println!("{:?}", result); // 1
}
