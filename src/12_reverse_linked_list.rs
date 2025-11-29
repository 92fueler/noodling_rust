/**
 * 296. reverse linked list
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
    pub fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {}
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
    // 1 -> 2 -> 3
    let ll: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    println!("{:?}", ll);

    let new_ll = Solution::reverse_linked_list(ll);

    println!("{:?}", new_ll);
}
