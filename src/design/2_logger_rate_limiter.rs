/**
 * 359. logger rate limiter
 *
 * Each unique message should only be printed at most every 10 seconds
 *
 * Implement Logger
 * - Logger() initializes the logger object
 * - bool should_print_message(), returns true if the message should be printed in the given timestamp,
 *   otherwise, returns false.
 *
 */
use std::collections::HashMap;

#[derive(Debug)]
struct Logger {
    last_printed: HashMap<String, i32>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            last_printed: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(&last_printed_timestamp) = self.last_printed.get(&message) {
            if timestamp - last_printed_timestamp < 10 {
                return false;
            }
        }

        self.last_printed.insert(message, timestamp);
        true
    }
}

fn main() {
    let mut logger = Logger::new();

    println!("{}", logger.should_print_message(1, "foo".to_string())); // true
    println!("{}", logger.should_print_message(2, "bar".to_string())); // true
    println!("{}", logger.should_print_message(3, "foo".to_string())); // false
    println!("{}", logger.should_print_message(11, "foo".to_string())); // true
}
