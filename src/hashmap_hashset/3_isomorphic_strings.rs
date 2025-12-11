/**
 * 205. isomorphic strings
 *
 * given two strings s and t, determine if they are isomorphic.
 *
 * Two strings are isomorphic if you can replace each character in s
 * with a corresponding character in t consistently throughout the string.
 *
 * input: s = "egg", t = "add"
 * output: true
 */
use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        // check if sc already has a mapping
        if let Some(mapped) = s_to_t.get(&sc) {
            if *mapped != tc {
                return false;
            }
        } else {
            s_to_t.insert(sc, tc);
        }

        // check if tc already has a mapping
        if let Some(mapped) = t_to_s.get(&tc) {
            if *mapped != sc {
                return false;
            }
        } else {
            t_to_s.insert(tc, sc);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_isomorphic_true() {
        assert_eq!(
            is_isomorphic("paper".to_string(), "title".to_string()),
            true
        );
    }

    #[test]
    fn test_is_isomorphic_false() {
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
    }
}
