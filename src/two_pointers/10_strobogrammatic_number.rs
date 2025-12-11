/**
 * 246. strobogrammatic number
 *
 * Given a string num which represents an integer,
 * return true if num is a strobogrammatic number.
 *
 * A strobogrammatic number is a number that looks the same when rotated 180 degrees
 * (looked at upside down).
 *
 */
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strobogrammatic() {
        assert_eq!(Solution::is_strobogrammatic(String::from("69")), true);
        assert_eq!(Solution::is_strobogrammatic(String::from("88")), true);
        assert_eq!(Solution::is_strobogrammatic(String::from("962")), false);
        assert_eq!(Solution::is_strobogrammatic(String::from("1")), true);
        assert_eq!(Solution::is_strobogrammatic(String::from("0")), true);
    }
}
