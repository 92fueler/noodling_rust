/**
 * 290. word pattern
 *
 * Given a pattern and a string s,
 * find if s follows the same patterns.
 *
 * Input: pattern = "abba", s = "dog cat cat dog"
 * Output: true
 */
use std::collections::HashMap;

struct Solution;

// Why &&str?
// The double reference happens because:
// words is Vec<&str> - contains borrowed string slices
// .iter() borrows each element → gives us &&str

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        // .split_whitespace() returns Iterator<Item =&str>
        let words: Vec<&str> = s.split_whitespace().collect();

        if pattern.chars().count() != words.len() {
            return false;
        }

        let mut p_to_w: HashMap<char, &str> = HashMap::new();
        let mut w_to_p: HashMap<&str, char> = HashMap::new();

        for (pc, word) in pattern.chars().zip(words.iter()) {
            // Check if pc has a mapping
            if let Some(&mapped) = p_to_w.get(&pc) {
                // p_to_w.get(&pc) returns Option<&&str>
                // mapped is &str
                // word is &&str
                // *word is &str
                if mapped != *word {
                    return false;
                }
            } else {
                p_to_w.insert(pc, word);
            }

            // Check reverse mapping
            if let Some(&mapped) = w_to_p.get(word) {
                // word is &&str
                // mapped is &char
                // pc is &char
                if mapped != pc {
                    return false;
                }
            } else {
                w_to_p.insert(word, pc);
            }
        }

        true
    }
}

fn main() {
    let pattern: String = "abba".to_string();
    let s: String = "dog cat cat dog".to_string();

    let result = Solution::word_pattern(pattern, s);

    println!("{}", result); // true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_false_case() {
        assert_eq!(
            Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
            false
        );
    }

    #[test]
    fn test_multiple_chars_to_one_word() {
        assert_eq!(
            Solution::word_pattern("aaaa".to_string(), "dog dog dog dog".to_string()),
            true
        );
    }
}
