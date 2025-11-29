use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {}

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let target: i32 = 9;
    let result: Vec<i32> = two_sum(nums, target);

    println!("Result: {:?}", result);

    let nums2: Vec<i32> = vec![3, 2, 4];
    let target2: i32 = 6;
    let result2: Vec<i32> = two_sum(nums2, target2);

    println!("Result2: {:?}", result2);
}
