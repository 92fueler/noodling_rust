/**
 * 977. squares of a sorted array
 *
 * Given an integer array non-decreasing
 * return an array of the squares of each number sorted
 *
 * 1. loop  (while left < right)
 * 2. don't mutate in-place in the loop
 */

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        if n == 0 {
            return Vec::new();
        }

        let mut result = vec![0; n];
        let mut left: usize = 0;
        let mut right: usize = n - 1;
        let mut p: usize = n - 1;

        loop {
            let left_abs = nums[left].abs();
            let right_abs = nums[right].abs();

            if left_abs > right_abs {
                result[p] = left_abs * left_abs;
                left += 1;
            } else {
                result[p] = right_abs * right_abs;
                right -= 1;
            }

            if p == 0 {
                break;
            }
            p -= 1;
        }

        result
    }
}

fn main() {
    let nums: Vec<i32> = vec![-4, -1, 0, 3, 10];
    // let nums = vec![10];

    let result: Vec<i32> = Solution::sorted_squares(nums); // [0, 9, 16, 100]

    println!("{:?}", result);
}
