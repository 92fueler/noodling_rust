/**
26. Remove Duplicates from Sorted Array

Given an interger array: non-decreasing order
remove the duplicates in-place
The relative order of the elements should be kept the same.
*/

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {}

fn main() {
    let mut v: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = remove_duplicates(&mut v);

    println!("Result: {:?}", result); // 5
}
