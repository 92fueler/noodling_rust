fn main() {
    let v = vec![1, 2, 100, 4, 5];
    // let idx = v.iter().find(|&x| *x == 100).unwrap();

    // let new: Vec<&i32> = v.iter().filter(|&x| *x > 5).collect();
    let new: Vec<i32> = v.iter().filter(|&x| *x > 5).map(|&x| x * 2).collect();

    println!("{:?}", new);
}
