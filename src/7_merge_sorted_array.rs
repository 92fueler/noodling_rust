/**
88. Merge Sorted Array

Given two sorted array in non-decreasing order

*/

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {}

fn main() {
    let mut num1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let mut num2: Vec<i32> = vec![2, 5, 6];
    let m = 3;
    let n = 3;

    merge(&mut num1, m, &mut num2, n);

    println!("result: {:?}", &num1);
}
