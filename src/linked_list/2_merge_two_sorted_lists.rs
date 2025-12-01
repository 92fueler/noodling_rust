/**
 * 21. merge two sorted lists
 *
 * Given the heads of two sorted linked lists
 *
 * return the head of the merged linked list
 *
 * 1. mutable local copies
 * 2. dummy node + curr node for traversal and both nodes should be mutable
 * 3. as_ref() vs. as_mut()
 * 4. take()
 */
#[derive(Clone, Debug)]
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;

        let mut dummy = Box::new(ListNode {
            val: -1,
            next: None,
        });

        let mut curr = &mut dummy;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let mut node = l1.take();
                l1 = node.as_mut().unwrap().next.take();
                curr.next = node;
            } else {
                let mut node = l2.take();
                l2 = node.as_mut().unwrap().next.take();
                curr.next = node;
            }

            curr = curr.next.as_mut().unwrap();
        }

        curr.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}

fn main() {
    let ll1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let ll2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 5, next: None })),
        })),
    }));

    let result = Solution::merge_two_lists(ll1, ll2);

    println!("{:?}", result);
}
