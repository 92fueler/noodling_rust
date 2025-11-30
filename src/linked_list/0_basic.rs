/**
 * Linked List
 * 1. definition
 * 2. traverse a list
 *
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

fn main() {
    // in Python:
    // while stops when node is None
    // while node:
    //     node = node.next

    // immutable traversal
    let mut curr = &head;

    // while curr is Some, update curr to point to next
    while let Some(node) = curr {
        curr = &node.next;
    }

    // while stops when node is the last node
    // while node and node.next:
    //     node = node.next

    // mutable traversal
    let mut curr = &mut head;

    while let Some(node) = curr {
        if node.next.is_node() {
            break; // after the break, node is the last node
        }

        curr = &mut node.next;
    }

    // dummy node with mutable borrowing pattern
    // allows:
    // easy insertion at head, easy deletion of the first node
    // curr is a mutable pointer to a pointer. This is how to mutate .next fields safely
    let mut dummy: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: head }));

    let mut curr = &mut dummy;

    // 1. move curr to next node
    curr = &mut curr.as_mut().unwrap().next; // move only one step
    // curr is &mut             Option<Box<ListNode>>
    // curr.as_mut().unwrap()   &mut Box<ListNode>>
    // .next
    // &mut next                becomes the next curr

    // 2. move all the way to the end
    while let Some(node) = curr {
        curr = &mut node.next;
    }
}
