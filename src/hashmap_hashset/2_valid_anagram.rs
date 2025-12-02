/**
 *
 * 242. valid anagram
 *
 * Given two strings s and t, return true if t is an anagram of s, and false otherwise.
 */
use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut char_count = HashMap::new();

    // Count s and t in one loop
    for (ch_s, ch_t) in s.chars().zip(t.chars()) {
        *char_count.entry(ch_s).or_insert(0) += 1;
        *char_count.entry(ch_t).or_insert(0) -= 1;
    }

    char_count.values().all(|&count| count == 0)
}

fn main() {
    let s: String = String::from("anagram");
    let t: String = String::from("nagaram");
    let result: bool = is_anagram(s, t);

    println!("{:?}", result);
}
