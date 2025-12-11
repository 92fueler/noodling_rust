/**
 * 58. length of last word
 *
 * Given a string s consisting of words and spaces
 * return the length of the last word in the string
 *
 * s = "Hello World  "                    // String

s.split(" ")
// Iterator yields: "Hello", "World", "", ""
// Type: Split<'_, &str>
// Item type: &str

.filter(|s| !s.is_empty())
// Iterator yields: "Hello", "World"
// Type: Filter<Split<...>, Closure>
// Item type: &str

.map(|s| s.len())
// Iterator yields: 5, 5
// Type: Map<Filter<...>, Closure>
// Item type: usize

.max()
// Consumes iterator, finds max: Some(5)
// Type: Option<usize>
*/

struct Solution;

impl Solution {
    // .split_whitespace() can handle multiple whitespaces
    // .split()
    pub fn length_of_last_word_1(s: String) -> i32 {
        s.split_whitespace().last().unwrap_or("").len() as i32
    }

    pub fn length_of_last_word_2(s: String) -> i32 {
        s.split(" ")
            .filter(|w| !w.is_empty())
            .map(|w| w.len() as i32)
            .max()
            .unwrap()
    }

    pub fn length_of_last_word_3(s: String) -> i32 {
        // iterate from right to left
        if s.is_empty() {
            return 0;
        }

        let n: usize = s.len();
        let s_chars: Vec<char> = s.chars().collect();
        let mut idx: usize = n - 1;
        let mut length: i32 = 0;

        // skip trailing whitespace
        while s_chars[idx] == ' ' {
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        while s_chars[idx] != ' ' {
            length += 1;
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        length
    }
}

fn main() {
    let s1: String = String::from("   fly me   to   the moon  ");
    let result1: i32 = Solution::length_of_last_word_1(s1);

    println!("{}", result1);

    let s2: String = String::from("  hello& World.  jian    wu  *athome   ");
    let result2: i32 = Solution::length_of_last_word_2(s2);

    println!("{}", result2);

    // edge case: "a"
    let s3: String = String::from("a");
    let result3: i32 = Solution::length_of_last_word_3(s3);

    println!("{}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s: String = String::from("a");
        assert_eq!(Solution::length_of_last_word_1(s), 1);
    }

    #[test]
    fn test_2() {
        let s: String = String::from("a");
        assert_eq!(Solution::length_of_last_word_2(s), 1);
    }

    #[test]
    fn test_3() {
        let s: String = String::from("a");
        assert_eq!(Solution::length_of_last_word_3(s), 1);
    }
}
