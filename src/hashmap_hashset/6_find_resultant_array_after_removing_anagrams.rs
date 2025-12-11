/**
 * 2273. find resultant array after removing anagrams
 *
 * Remove consecutive anagrams from teh array
 * Keep the first word of each consecutive group of anagrams,
 * remove the rest
 *
 */

struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut prev_sorted = String::new();

        for w in words {
            let mut w_chars: Vec<char> = w.chars().collect();
            w_chars.sort_unstable();
            let curr_sorted: String = w_chars.into_iter().collect();

            if curr_sorted != prev_sorted {
                result.push(w);
                prev_sorted = curr_sorted;
            }
        }

        result
    }
}

fn main() {
    // edge case: ["a", "b", "a"]
    let words: Vec<String> = vec!["a".to_string(), "b".to_string(), "a".to_string()];

    let result = Solution::remove_anagrams(words);

    println!("{:?}", result);
}
