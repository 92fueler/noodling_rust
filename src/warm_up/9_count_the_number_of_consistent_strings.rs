/**
 * 1684. Count the Number of Consistent Strings
 *
 * A string is consistent if all characters in the string appear in the string allowed.
 */
use std::collections::HashSet;

struct Solution;

// intuitive approach
// Rust native approach
impl Solution {
    pub fn count_consistent_strings_1(allowed: String, words: Vec<String>) -> i32 {
        let allowed_set: HashSet<char> = allowed.to_lowercase().chars().collect();
        let mut result: i32 = 0;

        for w in &words {
            let w_set: HashSet<char> = w.to_lowercase().chars().collect();
            // Check if w_set is a subset of allowed_set
            if w_set.is_subset(&allowed_set) {
                result += 1;
            }
        }

        result
    }

    pub fn count_consistent_strings_2(allowed: String, words: Vec<String>) -> i32 {
        let mut res: i32 = 0;
        let allowed_bytes: &[u8] = allowed.as_bytes();

        for w in &words {
            if w.bytes().all(|b| allowed_bytes.contains(&b)) {
                res += 1;
            }
        }

        res
    }
}

fn main() {
    let allowed: String = String::from("ab");
    let words: Vec<String> = vec![
        "ad".to_string(),
        "bd".to_string(),
        "aaab".to_string(),
        "baa".to_string(),
        "badab".to_string(),
    ];

    let result = Solution::count_consistent_strings(allowed, words);

    println!("{:?}", result); // 2
}
