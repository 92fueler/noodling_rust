/**
 * 296. reverse linked list
 *
 * Given the head of a singly linked list, reverse
 * return the reversed list.
 *
 * prev and curr pattern
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

// approach 1: iteration
impl Solution {
    pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head; // curr is head!!!

        // while curr has some val,
        // get the next node
        // curr.next -> prev
        // prev = curr
        // curr = curr.next
        while let Some(mut node) = curr {
            // curr is Option<Box<ListNode>
            // node is mutable Box<ListNode>
            let next_node = node.next.take(); // take the remaining ownership
            node.next = prev;
            prev = Some(node); // node is moved!!!
            curr = next_node;
        }

        prev
    }
}

// approach 2: recursion
// impl Solution {
//     pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         Self::reverse_recursive(head, None)
//     }

//     fn reverse_recursive(
//         head: Option<Box<ListNode>>,
//         prev: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         match head {
//             None => prev,
//             Some(mut node) => {
//                 let next = node.next.take();
//                 node.next = prev;
//                 Self::reverse_recursive(next, Some(node))
//             }
//         }
//     }
// }

fn main() {
    // 1 -> 2 -> 3 -> 4 -> 5
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

    println!("{:?}", ll);

    let new_ll = Solution::reverse_linked_list(ll);

    println!("{:?}", new_ll);
}
