/**
 * 917. reverse only letters
 *
 * Given a string s, reverse the string according to the following rules:
 * All the characters that are not English letters remain in the same position.
 * All the English letters (lowercase or uppercase) should be reversed.
 *
 */
struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        // left, right pointers
        // if  c is alphabetic, skip
        // if c is alphabetic, then swap

        if s.is_empty() {
            return String::new();
        }

        let mut v_chars: Vec<char> = s.chars().collect();

        let mut left: usize = 0;
        let mut right: usize = v_chars.len() - 1;

        // edge case, left = 0, right = 1
        while left < right {
            while left < right && !v_chars[left].is_alphabetic() {
                left += 1;
            }

            while left < right && !v_chars[right].is_alphabetic() {
                right -= 1;
            }

            v_chars.swap(left, right);
            left += 1;
            right -= 1;
        }

        let new_str: String = v_chars.iter().collect();
        new_str
    }
}

fn main() {
    let s: String = String::from("Test1ng-Leet=code-Q!");
    // let s: String = String::from("ab");
    let result: String = Solution::reverse_only_letters(s);

    println!("{:?}", result); // "Qedo1ct-eeLg=ntse-T!"
}
