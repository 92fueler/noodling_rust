/**
 * 1684. Count the Number of Consistent Strings
 *
 * A string is consistent if all characters in the string appear in the string allowed.
 */

// intuitive approach
// Rust native approach
impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {}
}

fn main() {
    let allowed: String = String::from("ab");
    let words: Vec<String> = vec!["ad", "bd", "aaab", "baa", "badab"];

    let result = Solution::count_consistent_strings(allowed, words);

    println!("{:?}", result);
}
