/**
 * 408. valid word abbreviation
 *
 * Given a string word and an abbreviation abbr,
 * return whether the string matches the given abbreviation.
 */

struct Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {}
}

fn main() {
    let word = String::from("internationalization");
    let abbr = String::from("i12iz4n");

    let result = Solution::valid_word_abbreviation(word, abbr);

    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_word_abbreviation() {
        let word = String::from("internationalization");
        let abbr = String::from("i12iz4n");
        assert!(Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_invalid_abbreviation() {
        let word = String::from("internationalization");
        // Invalid: 'x' doesn't match the character at that position
        let abbr = String::from("i5x11o1");
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_invalid_abbreviation_too_long() {
        let word = String::from("internationalization");
        // Invalid: skips too many characters (21 > 20)
        let abbr = String::from("i5a12o1");
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_invalid_abbreviation_too_short() {
        let word = String::from("internationalization");
        // Invalid: doesn't cover all characters (19 < 20)
        let abbr = String::from("i5a11o");
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_invalid_leading_zero() {
        let word = String::from("word");
        // Invalid: leading zero not allowed
        let abbr = String::from("w02d");
        assert!(!Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_exact_match() {
        let word = String::from("word");
        let abbr = String::from("word");
        assert!(Solution::valid_word_abbreviation(word, abbr));
    }

    #[test]
    fn test_all_numbers() {
        let word = String::from("word");
        let abbr = String::from("4");
        assert!(Solution::valid_word_abbreviation(word, abbr));
    }
}
