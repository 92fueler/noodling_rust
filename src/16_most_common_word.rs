/**
 * 819. most common word
 *
 * Give a string and a string array of banned words
 *
 * return the most frequent word that is not banned.
 */

struct Solution;

impl Solution {
    pub fn most_common_words(paragraph: String, banned: Vec<String>) -> String {}
}

fn main() {
    let s: String = String::from("Bob hit a ball, the hit BALL flew far after it was hit.");

    let banned: Vec<String> = vec!["hit"];

    let result: String = Solution::most_common_words(s, banned);

    println!("{:?}", result);
}
