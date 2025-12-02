/**
 * 205. isomorphic strings
 *
 * given two strings s and t, determine if they are isomorphic.
 *
 * input: s = "egg", t = "add"
 * output: true
 */
use std::collections::{HashMap, HashSet};

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut mapping: HashMap<char, char> = HashMap::new();
    let mut used: HashSet<char> = HashSet::new();

    for (ch_s, ch_t) in s.chars().zip(t.chars()) {
        if let Some(&mapped_char) = mapping.get(&ch_s) {
            // Character already has a mapping
            if mapped_char != ch_t {
                return false;
            }
        } else {
            // New mapping - check if ch_t is already used
            if used.contains(&ch_t) {
                return false;
            }
            mapping.insert(ch_s, ch_t);
            used.insert(ch_t);
        }
    }

    true
}

fn main() {
    let s: String = String::from("egg");
    let t: String = String::from("add");

    let result: bool = is_isomorphic(s, t);

    println!("{:?}", result);
}
