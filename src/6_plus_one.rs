/**
66. Plus one

Given a vector of digits i32
digit: 0 - 9
[3, 4, 5, 9] -> [3, 4, 6, 0]

+ 1

returns a vector of digits
*/

// 1. implicit carry
// 2. explicit carry
fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {}

fn main() {
    let v1: Vec<i32> = vec![9];
    let result1: Vec<i32> = plus_one(v1);

    println!("Result1: {:?}", result1);

    let v2: Vec<i32> = vec![9, 9];
    let result2: Vec<i32> = plus_one(v2);

    println!("Result2: {:?}", result2);
}
