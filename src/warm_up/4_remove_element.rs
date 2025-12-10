/**
 * 27. Remove Element
 *
 * Given an integer array: nums and an integer: val
 * remove all occurrences of val in nums in place
*/

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {}

fn main() {
    // let mut v1: Vec<i32> = vec![3, 2, 2, 3];
    // let val1: i32 = 3;
    // let result1: i32 = remove_element(&mut v1, val1);

    // println!("Result1: {}", result1);

    let mut v2: Vec<i32> = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val2: i32 = 2;
    let result2: i32 = remove_element(&mut v2, val2);

    println!("Result2: {}", result2);
}
