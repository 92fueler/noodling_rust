/**
 * 3. longest substring without repeating characters
 *
 * Given a string s,
 * find the length of the longest substring without duplicate characters.
 *
 * Input: s = "pwwkew"
 * Output: 3
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash_map: HashMap<char, usize> = HashMap::new();
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut result: i32 = 0;

        let chars: Vec<char> = s.chars().collect();

        while right < chars.len() {
            let curr_char = chars[right];

            if let Some(&prev_idx) = hash_map.get(&curr_char) {
                if prev_idx >= left {
                    left = prev_idx + 1;
                }
            }

            hash_map.insert(curr_char, right);
            result = result.max((right - left + 1) as i32);
            right += 1
        }

        result
    }
}

fn main() {
    let s: String = "abcabcbb".to_string();
    let result: i32 = Solution::length_of_longest_substring(s);

    println!("{}", result); // 3
}
