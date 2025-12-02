fn main() {
    // create
    let mut s: String = String::from("hello");

    // concatenate
    s += " world";
    s.push('!');

    // insert vs. insert_str
    s.insert_str(0, "from wu ");

    // pop
    let popped = s.pop().unwrap();
    println!("{}", popped);

    // remove
    s.remove(3);

    // position()
    // find()
    //

    // iterate

    // iterate, transform,

    // split, transform, rejoin

    let parts = &s[0..3];

    for c in parts.chars() {
        println!("{}", c);
    }

    // Vec<char> to String
    let chars: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    let result: String = chars.iter().collect();

    println!("{:?}", s);
}
