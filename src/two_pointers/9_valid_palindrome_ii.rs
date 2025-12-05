/**
 * 680. valid palindrome II
 *
 * Given a string s,
 * return true if the s can be palindrome
 * after deleting at most one character from it.
 */
struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome_basic() {
        let word = String::from("aba");
        assert!(Solution::valid_palindrome(word));
    }

    #[test]
    fn test_valid_palindrome_advance() {
        let word = String::from("abca");
        assert!(Solution::valid_palindrome(word));
    }
}
