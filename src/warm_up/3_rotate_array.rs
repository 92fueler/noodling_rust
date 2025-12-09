/**
 * 189. rotate array
 *
 * Given an integer array nums
 * rotate the array to the right by k steps
 *
 * Input: nums = [1,2,3,4,5,6,7], k = 3
 * Output: [5,6,7,1,2,3,4]
 */
struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n: usize = nums.len();
        let k: usize = (k as usize) % n; // Need to cast k to usize

        if k == 0 {
            return; // No rotation needed
        }

        nums.reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_array() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k: i32 = 3;

        Solution::rotate(&mut nums, k); // rotate returns (), not a Vec
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_full_cycle() {
        let mut nums = vec![1, 2, 3];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![1, 2, 3]); // k = n, no change
    }

    #[test]
    fn test_rotate_larger_than_n() {
        let mut nums = vec![1, 2, 3];
        Solution::rotate(&mut nums, 4); // 4 % 3 = 1
        assert_eq!(nums, vec![3, 1, 2]);
    }
}
