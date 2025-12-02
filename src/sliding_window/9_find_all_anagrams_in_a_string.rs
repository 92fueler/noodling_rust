/**
 * 438. find all anagrams in a string
 *
 * Given two strings s and p,
 * return an array of all the start indices of p's anagrams in s.
 *
 */

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {}
}

fn main() {
    let s: String = String::from("cbaebabacd");
    let p: String = String::from("abc");

    let result: Vec<i32> = Solution::find_anagram(s, p);

    println!("{:?}", result);
}
