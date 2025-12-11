/**
 * 27. Remove Element
 *
 * Given an integer array: nums and an integer: val
 * remove all occurrences of val in nums in place
 *
 * return the length of the remaining array
 *
 * Input: nums = [3,2,2,3], val = 3
 * Output: 2, nums = [2,2,_,_]
*/

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut slow: usize = 0;
    let mut fast: usize = 0;
    let n: usize = nums.len();

    while fast < n {
        if nums[fast] != val {
            nums.swap(slow, fast);
            slow += 1;
        }

        fast += 1
    }

    (slow) as i32
}

fn main() {
    let mut v: Vec<i32> = vec![3, 2, 2, 3];
    let val: i32 = 3;
    let result: i32 = remove_element(&mut v, val);

    println!("Result1: {}", result1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut v: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];

        assert_eq!(remove_element(&mut v, 2), 5);
    }
}
