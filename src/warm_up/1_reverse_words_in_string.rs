/**
 * 151. reverse words in a string
 * Given an input string s, reverse the order of the words.
 *
 * Input: s = "the sky is blue"
 * Output: "blue is sky the"
 *
 * 186. reverse words in a string II
 * Given a character array s, reverse the order of the words
 *
 * Input: s = ["t","h","e"," ","s","k","y"," ","i","s"," ","b","l","u","e"]
 * Output: ["b","l","u","e"," ","i","s"," ","s","k","y"," ","t","h","e"]
 *
 * 557. Reverse words in a string III
 * Given an input string s, reverse the order of the words.
 *
 * Input: s = "Let's take LeetCode contest"
 * Output: "s'teL ekat edoCteeL tsetnoc"
 */

// This is the parent module (the file's root module)
struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    pub fn reverse_words_ii(s: &mut Vec<char>) {
        let mut start_idx = 0;

        for idx in 0..s.len() {
            if s[idx] == ' ' {
                s[start_idx..idx].reverse();
                start_idx = idx + 1;
            }
        }

        s[start_idx..].reverse();
        s.reverse();
    }

    pub fn reverse_words_iii(s: String) -> String {
        // w                            &str (a string slice, not owned)
        // w.chars()                    Chars (an iterator over chars, NOT Vec<char>)
        // w.chars().rev()              Rev<Chars> (a reversed iterator, NOT Vec<char>)
        // w.chars().rev().collect()    String (collects iterator into String)
        s.split_whitespace()
            .map(|w| w.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }

    // pub fn reverse_words_iii(s: String) -> String {
    //     s.split_whitespace()
    //         .map(|&w| w.chars().rev().collect::<String>()) // <-- ERROR
    //         .collect::<Vec<String>>()
    //         .join(" ")
    // }
}

// This is a child module nested inside the parent
#[cfg(test)]
mod tests {
    use super::*; // Import everything from parent (Solution struct)

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the"
        );
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello"
        );
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a"
        );
    }

    #[test]
    fn test_reverse_words_ii() {
        let mut s = vec![
            't', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e',
        ];
        Solution::reverse_words_ii(&mut s);
        assert_eq!(
            s,
            vec![
                'b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e'
            ]
        );

        let mut s2 = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_words_ii(&mut s2);
        assert_eq!(s2, vec!['h', 'e', 'l', 'l', 'o']);
    }

    #[test]
    fn test_reverse_words_iii() {
        assert_eq!(
            Solution::reverse_words_iii("Let's take LeetCode contest".to_string()),
            "s'teL ekat edoCteeL tsetnoc"
        );
        assert_eq!(
            Solution::reverse_words_iii("God Ding".to_string()),
            "doG gniD"
        );
        assert_eq!(Solution::reverse_words_iii("hello".to_string()), "olleh");
    }
}

// fn main() {
//     let s: String = String::from("the sky is blue");

//     let result: String = Solution::reverse_words(s);

//     let s: String = String::from("Let's take LeetCode contest");
//     let result: String = Solution::reverse_words_iii(s);

//     println!("{:?}", result);
// }
