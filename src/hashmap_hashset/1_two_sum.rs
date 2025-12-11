/**
 * 1. two sum
 *
 * Input: nums = [2,7,11,15], target = 9
 * Output: [0,1]
 */
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // key: num, value: idx as i32
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
        let complement = target - num;
        if hash_map.contains_key(&complement) {
            return vec![*hash_map.get(&complement).unwrap(), idx as i32];
        }

        hash_map.insert(*num, idx as i32);
    }

    vec![]
}

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = two_sum(nums, target);

    println!("Result: {:?}", result); // [1, 2]

    let nums2: Vec<i32> = vec![3, 2, 4];
    let target2: i32 = 6;
    let result2: Vec<i32> = two_sum(nums2, target2);

    println!("Result2: {:?}", result2); // [0, 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_second_case() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        assert_eq!(two_sum(nums, target), vec![2, 4]);
    }

    #[test]
    fn test_with_zero() {
        let nums = vec![0, 4, 3, 0];
        let target = 0;
        assert_eq!(two_sum(nums, target), vec![0, 3]);
    }
}
