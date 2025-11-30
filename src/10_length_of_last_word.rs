/**
 * 58. length of last word
 *
Given a string s consisting of words and spaces
return the length of the last word in the string

 * s = "Hello World  "                    // String

s.split(" ")
// Iterator yields: "Hello", "World", "", ""
// Type: Split<'_, &str>
// Item type: &str

.filter(|s| !s.is_empty())
// Iterator yields: "Hello", "World"
// Type: Filter<Split<...>, Closure>
// Item type: &str

.map(|s| s.len())
// Iterator yields: 5, 5
// Type: Map<Filter<...>, Closure>
// Item type: usize

.max()
// Consumes iterator, finds max: Some(5)
// Type: Option<usize>

.unwrap()
// Extracts value: 5
// Type: usize

as i32
// Casts to: 5
// Type: i32
*/

// .split_whitespace() can handle multiple whitespaces
// .split()
pub fn length_of_last_word(s: String) -> i32 {}

fn main() {
    let s1: String = String::from("   fly me   to   the moon  ");
    let result1: i32 = length_of_last_word(s1);

    println!("{}", result1);

    let s2: String = String::from("  hello& World.  jian    wu  *athome   ");
    let result2: i32 = length_of_last_word(s2);

    println!("{}", result2);
}
