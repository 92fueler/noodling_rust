/**
 * 26. Remove Duplicates from Sorted Array
 *
 * Given an integer array: non-decreasing order
 *
 * remove the duplicates in-place
 * The relative order of the elements should be kept the same.
 *
 * Input: nums = [1,1,2]
 * Output: 2, nums = [1,2,_]
*/

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let n: usize = nums.len();
    let mut slow: usize = 0;
    let mut fast: usize = 0;

    while fast < n {
        if nums[slow] != nums[fast] {
            slow += 1;
            nums.swap(slow, fast);
        }

        fast += 1;
    }

    (slow + 1) as i32
}

fn main() {
    let mut v: Vec<i32> = vec![1, 1, 2];
    let result = remove_duplicates(&mut v);

    println!("Result: {:?}", result); // 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut v: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut v), 5);
    }
}
