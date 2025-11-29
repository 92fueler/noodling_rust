/**
35. Search Insert Position

nums: sorted, distinct integers
target: i32

return the index if the target is found
if not, return the index where it would be if it were inserted in order
*/

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {}

fn main() {
    let nums: Vec<i32> = vec![1, 1, 3, 5, 5, 6];
    let target: i32 = 5;

    let result: i32 = search_insert(nums, target);
    // println!("{:?}", nums);

    println!("Result: {}", result); // 3
}
