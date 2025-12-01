/**
 * 485. max consecutive ones
 *
 * Given a binary array nums,
 * return the maximum number of consecutive 1's in the array.
 */

struct Solution;

// approach 1: generic
// approach 2: vector transformation
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {}
}

fn main() {
    let nums: Vec<i32> = vec![1, 1, 0, 1, 1, 1];
    let result: i32 = Solution::find_max_consecutive_ones(nums);

    println!("{:?}", result);
}
