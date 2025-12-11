/**
 * 266. palindrome permutation
 *
 * Give a string s,
 * return true if a permutation of s could form a palindrome
 * and false otherwise
 *
 * Input: s = "aab"
 * Output: true
 *
 */
// For a string to form a palindrome:
// Even length: Every character must appear an even number of times
// Odd length: Exactly one character can appear an odd number of times
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut freq = HashMap::new();

        // Count frequency of each character
        for ch in s.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }

        // Count how many characters have odd frequency
        let odd_count = freq.values().filter(|&&count| count % 2 == 1).count();
        // freq.values() -> Returns: Values<'_, char, i32>
        // Think of it as: Iterator<Item = &i32>

        // *Why double &&?
        // The iterator yields &i32 (reference to value in HashMap)
        // The closure parameter itself is a reference: |param| where param = &&i32
        // So we need |&&count| to fully destructure to i32

        // At most one character can have odd frequency
        odd_count <= 1
    }
}

fn main() {
    let s: String = "aab".to_string();
    let result = Solution::can_permute_palindrome(s);

    println!("{}", result);
}
