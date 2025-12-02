/**
 * 83. remove duplicates from sorted list
 *
 *
 * head: Option<Box<ListNode>>
 * head.as_mut: Option<&mut Box<ListNode>>
 * curr: o
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
        let mut head = head;
        let mut curr = &mut head;

        while let Some(node) = curr {
            while node.next.is_some() && node.val == node.next.as_ref().unwrap().val {
                node.next = node.next.as_mut().unwrap().next.take();
            }

            curr = &mut node.next;
        }

        head
    }

    // IMPORTANT
    // fn bad(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut head = head;
    //     let mut curr = &mut head;

    //     // Rust is complaining that curr (internally shown as curr.0)
    //     // is mutably borrowed across iterations of the loop.
    //     while let Some(ref mut node) = curr {
    //         // node is mutable ref -> Box<ListNode>
    //         if let Some(ref mut next) = node.next {
    //             // next is mutable ref -> Box<ListNode>
    //             if node.val == next.val {
    //                 node.next = next.next.take();
    //             } else {
    //                 curr = &mut node.next; // <-- ERROR
    //             }
    //         } else {
    //             // no node.next
    //             break;
    //         }
    //     }

    //     head
    // }
}

fn main() {
    // 1 -> 2 -> 3 -> 3 -> 5
    let ll: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
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

    println!("{:?}", result); // 1 -> 2 -> 3 -> 5
}
