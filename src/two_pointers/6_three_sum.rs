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
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort(); // sort

        let mut result: Vec<Vec<i32>> = vec![];

        let n: usize = nums.len();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                break;
            }

            let mut left: usize = i + 1;
            let mut right: usize = n - 1;

            while left < right {
                let curr_sum = nums[i] + nums[left] + nums[right];

                if curr_sum == 0 {
                    result.push(vec![i as i32, left as i32, right as i32]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if curr_sum > 0 {
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }

        result
    }
}

fn main() {
    let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];

    let result: Vec<Vec<i32>> = Solution::three_sum(nums);
    println!("{:?}", result);
}
