/**
 * 819. most common word
 *
 * Give a string and a string array of banned words
 * return the most frequent word that is not banned.
 */
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn most_common_words(paragraph: String, banned: Vec<String>) -> String {
        let banned: HashSet<String> = banned.into_iter().collect();

        let cleaned_paragraph: String = paragraph
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphabetic() { c } else { ' ' })
            .collect();

        let mut hash_map: HashMap<String, i32> = HashMap::new();

        for w in cleaned_paragraph.split_whitespace() {
            let word: String = w.to_string();

            if !banned.contains(&word) {
                *hash_map.entry(word).or_insert(0) += 1;
            }
        }
        // approach 1: rust idiomatic approach
        // hash_map
        //     .into_iter()
        //     .max_by_key(|(_, v)| *v)
        //     .map(|(w, _)| w)
        //     .unwrap_or_default()

        // generic
        let mut max_val: i32 = 0;
        let mut result: String = String::new();

        for (k, v) in hash_map.into_iter() {
            if v > max_val {
                max_val = v;
                result = k.to_string();
            }
        }

        // using .iter() vs. .into_iter()

        // for (k, v) in hash_map.iter() {
        //     if *v > max_val {
        //         max_val = *v;
        //         result = k.to_string();
        //     }
        // }

        result
    }
}

fn main() {
    let s: String = String::from("Bob hit a ball, the hit BALL flew far after it was hit.");
    let banned: Vec<String> = vec![String::from("hit")];
    let result: String = Solution::most_common_words(s, banned);

    println!("{:?}", result); // expect "ball"
}
