/**
 * 225. implement stack using queues
 *
 */
use std::collections::VecDeque;

struct MyStack {
    queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let size = self.queue.len();

        for _ in 0..size - 1 {
            let front = self.queue.pop_front().unwrap();
            self.queue.push_back(front);
        }

        self.queue.pop_front().unwrap()
    }

    fn top(&mut self) -> i32 {
        let result = self.pop();
        self.push(result);
        result
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn main() {
    let my_stack = MyStack::new();
}
