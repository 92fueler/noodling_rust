/**
 * https://exercism.org/tracks/rust/exercises/clock/edit
 *
 * Clock
 *
 * Implement a clock that handles times without dates.
 * You should be able to add and subtract minutes to it.
 * Two clocks that represent the same time should be equal to each other.
 *
 * Rust Traits for .to_string()
 * You will also need to implement .to_string() for the Clock struct.
 * We will be using this to display the Clock's state. You can either do it via implementing it directly or using the Display trait.
 * If so, try implementing the Display trait for Clock instead.
 * Traits allow for a common way to implement functionality for various types.
 *
 * Example:
 * let clock = Clock::new(23, 50);  // 11:50 PM
 * let after = clock.add_minutes(20);  // Add 20 minutes
 * println!("{}", after);  // "00:10" (past midnight!)
 *
 */
// Problem boilerplate code
// pub struct Clock;

// impl Clock {
//     pub fn new(hours: i32, minutes: i32) -> Self {
//         todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
//     }

//     pub fn add_minutes(&self, minutes: i32) -> Self {
//         todo!("Add {minutes} minutes to existing Clock time");
//     }
// }
use std::fmt;

// problem statement says
#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32, // Store total minutes since midnight (0-1439)
}

// Need to implement .to_string()
// let clock = Clock::new(10, 30);
// let s = clock.to_string();  // Need this to work!
// println!("{}", s);  // "10:30"

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized = total_minutes.rem_euclid(24 * 60); // Wrap around
        // keep the value within 0 - 1439 range and wrap around

        Clock {
            minutes: normalized,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }

    // pub fn to_string(&self) -> String {      <-- NOT RECOMMENDED
    //     let hours = self.minutes / 60;
    //     let mins = self.minutes % 60;
    //     format!("{:02}:{:02}", hours, mins)
    // }
}

// Implement the Display trait is RECOMMENDED!!!
// get .to_string() for free (automatically provided by Display)
// You can also use println!("{}", clock) directly
// You can use format!("{}", clock)
// More idiomatic and interoperable with Rust ecosystem
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // &self, borrows the Clock (doesn't take the ownership)
        // f, Mutable reference to the formatter
        // Formatter,  a type from std::fmt that writes formatted output
        //  <'_> - The Lifetime
        // Lifetime annotation that says "this reference is valid for some amount of time"
        // '_ is a lifetime placeholder - Rust fills it in automatically
        let hours = self.minutes / 60;
        let mins = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, mins) // Format as HH:MM
        // :02, display this number with at least 2 digits, padding with 0s if needed
    }
}
