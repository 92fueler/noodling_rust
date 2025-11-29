/**
 * 19. Remove nth Node from end of list
 *
 *
 * given the head of the linked list
 * remove the nth node from the end of the list
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {}
}

fn main() {
    // 1 -> 2 -> 3 -> 4 -> 5, n = 2

    let ll: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    let n: i32 = 2;

    println!("{:?}", ll);
    let result: Option<Box<ListNode>> = Solution::remove_nth_from_end(ll, n);

    println!("{:?}", result);
}
