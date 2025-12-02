use std::collections::VecDeque;

fn main() {
    // create
    let mut deque: VecDeque<i32> = VecDeque::new();

    // enqueue
    deque.push_back(1);
    deque.push_back(2);

    // dequeue
    deque.pop_front(); // returns Option<i32>

    // check if empty
    if deque.is_empty() {
        println!("empty");
    }

    // get length
    let length = deque.len();
}
