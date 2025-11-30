/**
 * 15. 3Sum
 *
 * Given an integer array, contains duplicates
 *
 * sum = 0
 * 3 <= nums.length <= 3000
 */

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {}
}

fn main() {
    let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];

    let result: Vec<Vec<i32>> = Solution::three_sum(nums);
    println!("{:?}", result);
}
