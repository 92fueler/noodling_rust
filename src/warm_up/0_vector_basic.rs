fn main() {
    let arr = [1, 2, 3];
    for x in arr { // x is i32, because arrays copy values
    }
    for x in arr.iter() { // x is &i32
    }
}
