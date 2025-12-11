/**
 * 485. max consecutive ones
 *
 * Given a binary array nums,
 * return the maximum number of consecutive 1's in the array.
 *
 * Input: nums = [1,1,0,1,1,1]
 * Output: 3
 */

struct Solution;

// approach 1: generic
// approach 2: vector transformation
impl Solution {
    pub fn find_max_consecutive_ones_1(nums: Vec<i32>) -> i32 {
        let mut max_len: i32 = 0;
        let mut idx: usize = 0;
        let n: usize = nums.len();

        while idx < n {
            if nums[idx] == 1 {
                let mut local_len: i32 = 0;
                while idx < n && nums[idx] == 1 {
                    local_len += 1;
                    idx += 1;
                }

                max_len = max_len.max(local_len);
            } else {
                idx += 1;
            }
        }

        max_len
    }

    pub fn find_max_consecutive_ones_2(nums: Vec<i32>) -> i32 {
        nums.split(|&n| n == 0)
            .map(|chunk| chunk.len() as i32)
            .max()
            .unwrap_or(0)
        // nums is Vec<i32>
        // .split() -> takes a predicate closure and returns an iterator over slices
        // Iterator<Item =&[i32]>
        // .map() -> input iterator of &[i32], each chunk is &[i32]
        // output: Iterator<Item =i32>
        // .max() -> consumes the iterator and finds the maximum value
        // output: Option<i32>, returns None if the iterator is empty
    }
}

fn main() {
    let nums: Vec<i32> = vec![1, 1, 0, 1, 1, 1];
    let result: i32 = Solution::find_max_consecutive_ones_1(nums);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_consecutive_ones_1() {
        let nums: Vec<i32> = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(Solution::find_max_consecutive_ones_1(nums), 3);
    }

    #[test]
    fn test_find_max_consecutive_ones_2() {
        let nums: Vec<i32> = vec![1, 1, 0, 1, 1, 1];
        assert_eq!(Solution::find_max_consecutive_ones_2(nums), 3);
    }
}
