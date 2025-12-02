fn main() {
    let mut stack: Vec<i32> = Vec::new();

    stack.push(1);
    stack.push(2);

    let top = stack.pop(); // returns Option<i32>

    let last = stack.last(); // returns Option<i32>

    if stack.is_empty() {
        println!("empty");
    }

    let length: usize = stack.len();

    // common pattern
    // built a Vec<char>
    let stack: Vec<char> = vec!['h', 'l'];

    // Checking last element before popping
    if let Some(&last) = stack.last() {
        if last == target {
            stack.pop();
        }
    }

    // Using while let for processing
    while let Some(top) = stack.pop() {
        // Process top
        if some_condition {
            break;
        }
    }
}
