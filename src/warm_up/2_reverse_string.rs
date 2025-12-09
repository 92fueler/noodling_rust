/**
 *
 * 344. reverse string
 * reverses a string in place with O(1) extra memory
 *
 * Input: s = ["h","e","l","l","o"]
 * Output: ["o","l","l","e","h"]
 *
 * 541. reverse string II
 *
 * Give a string s and an integer k,
 * reverse the first k characters for every 2k characters
 * If there are fewer than k characters left,
 * reverse all of them.
 * If there are less than 2k but greater than or equal to k characters,
 * then reverse the first k characters and leave the other as original.
 *
 * Input: s = "abcdefg", k = 2
 * Output: "bacdfeg"
 */

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n: usize = s.len();
        if n == 0 {
            return;
        }

        let mut left: usize = 0;
        let mut right: usize = s.len() - 1;

        //left is 0, right = 0
        while left <= right {
            s.swap(left, right);
            left += 1;

            if right == 0 {
                break;
            }

            right -= 1;
        }
    }

    pub fn reverse_str_ii(s: String, k: i32) -> String {
        let k = k as usize;
        let mut chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        // Process chunks of 2k
        let mut i = 0;
        while i < len {
            // Calculate the end of the reversal range
            let end = std::cmp::min(i + k, len);

            // Reverse the first k characters (or remaining if less than k)
            chars[i..end].reverse();

            // Move to the next 2k chunk
            i += 2 * k;
        }

        chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
    #[test]
    fn test_example_1() {
        let s = String::from("abcdefg");
        let k = 2;
        assert_eq!(Solution::reverse_str_ii(s, k), "bacdfeg");
        // Explanation:
        // "ab" -> "ba" (reverse first 2)
        // "cd" -> "cd" (keep next 2)
        // "ef" -> "fe" (reverse next 2)
        // "g" -> "g" (only 1 left, reverse it)
    }

    #[test]
    fn test_example_2() {
        let s = String::from("abcd");
        let k = 2;
        assert_eq!(Solution::reverse_str_ii(s, k), "bacd");
        // "ab" -> "ba" (reverse first 2)
        // "cd" -> "cd" (keep next 2)
    }

    #[test]
    fn test_less_than_k() {
        let s = String::from("abc");
        let k = 5;
        assert_eq!(Solution::reverse_str_ii(s, k), "cba");
        // Less than k characters, reverse all
    }

    #[test]
    fn test_exactly_k() {
        let s = String::from("abcde");
        let k = 3;
        assert_eq!(Solution::reverse_str_ii(s, k), "cbade");
        // "abc" -> "cba" (reverse first 3)
        // "de" -> "de" (less than 3 left, only 2)
    }

    #[test]
    fn test_single_char() {
        let s = String::from("a");
        let k = 1;
        assert_eq!(Solution::reverse_str_ii(s, k), "a");
    }

    #[test]
    fn test_large_k() {
        let s = String::from("abcdefgh");
        let k = 10;
        assert_eq!(Solution::reverse_str_ii(s, k), "hgfedcba");
        // k > length, reverse entire string
    }
}
