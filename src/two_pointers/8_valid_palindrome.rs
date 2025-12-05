/**
 * 125. valid palindrome
 *
 * Given a string s,
 * return true if it is a palindrome, or false otherwise.
 */

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut flag = true;
        // to_lowercase, alphanumeric
        let cleaned: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric())
            .collect();

        if cleaned.is_empty() {
            return flag;
        }

        let mut left = 0;
        let mut right = cleaned.len() - 1;

        while left < right {
            if cleaned[left] != cleaned[right] {
                flag = false;
            }
            left += 1;
            right -= 1;
        }

        flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_basic() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(Solution::is_palindrome(s));
    }
}
