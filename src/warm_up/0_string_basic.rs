/**
 * 1. create a new string (5)
 * 2. modify a string
 *      - concatenate (4)
 *      - insert (2)
 *      - pop - handle Option<char>
 *      - remove by idx
 *      - truncate
 *      - reverse a string (3)
 *      - uppercase or lowercase
 * 3. access by idx (4)
 * 4. find() vs. position()
 *      - find()        returns Option<usize>
 *      - position()    for search inside iterators
 * 5. string-level check
 *      - is_empty()
 *      - length
 *      - substring in string
 *      - starts with or ends with
 *      - check entire string:
 *          alphabetic, ascii, alphanumeric, numeric, ascii_digit,
 *          lowercase, uppercase, whitespace
 * 6. iterate: over bytes vs. chars
 * 7. transform
 *      - Given a string, split by comma and trim, then join back
 *      - Given a string, capitalize first letter of each word
 *
 *
 * 8. String <-> Vec conversion
 *      - String        -> Vec<char>
 *      - Vec<char>     -> String
 *      - String        -> [u8]
 *      - [u8]          -> String
 *      - String        -> Vec<&str>
 *      - Vec<&str>     -> String
 *      - Vec<String>   -> String
 *      - String        -> Vec<i32>  parse an integer string
 *      - Vec<i32>      -> String
 */

fn main() {
    // 1. create a new string (5)
    let s1: &str = "hello"; // string literal (immutable, stack/static)
    let s2: String = "hello".to_string(); // &str -> String (heap allocated)
    let s3: String = String::from("hello"); // same as to_string()
    let s4: String = String::new(); // empty String
    let s5: &str = &s3; // String -> &str (borrowing)

    // 2. modify a string
    // 2.1 concatenate (4)
    let mut s: String = String::from("hello");
    s += " world"; // append &str
    s.push('!'); // append single char
    s.push_str(" Jian"); // append &str
    println!("Concatenated: {}\n", s);

    let word1 = "hello";
    let word2 = "world";
    let result: String = format!("{} {}!", word1, word2);
    println!("Formatted: {}\n", result);

    // 2.2 insert (2)
    s.insert(10, '?'); // insert char at index (byte position)
    println!("After insert('?'): {}", s);

    s.insert_str(11, "[STR]"); // insert &str at index
    println!("After insert_str: {}\n", s);

    // 2.3 pop - handle Option<char> (4)
    // if let - cleanest for simple cases
    if let Some(c) = s.pop() {
        println!("Popped char: {:?}", c);
        println!("As string: {}", c.to_string());
    }

    // unwrap - panics if None (use when certain it's not empty)
    let mut s_certain = String::from("x");
    let c = s_certain.pop().unwrap();
    println!("Unwrapped: {}\n", c);

    // unwrap_or - provide default value
    let mut s_test = String::from("test");
    let c = s_test.pop().unwrap_or('_');

    // unwrap_or_else - lazy default (closure only runs if None)
    let mut s_empty = String::new();
    let c = s_empty.pop().unwrap_or_else(|| {
        println!("Empty string, using default!");
        '?'
    });

    // 2.4 remove by idx
    let removed_char = s.remove(10); // removes char at byte index, returns it
    println!("Removed char at index 10: {:?}", removed_char);
    println!("After remove: {}\n", s);

    // 2.5 reverse
    // text.reverse();  <-- ERROR!
    // ❌ This doesn't work:
    // let s = "hello";
    // s.reverse(); // ERROR: `&str` is immutable

    // ❌ This also doesn't work:
    // let mut s = String::from("hello");
    // s.reverse(); // ERROR: String doesn't implement .reverse()

    // reverse: chars vs. bytes
    let text = "hello";
    // let text = String::from("hello");
    let reversed: String = text.chars().rev().collect(); // text is &str
    let reversed: String = text.bytes().rev().map(|b| b as char).collect();
    println!("\nreversed: {}", reversed);

    let mut text_chars: Vec<char> = text.chars().collect();
    text_chars.reverse();
    println!("\ntext_chars: {:?}", text_chars);

    // 3. access by idx (4)
    let s = String::from("hello");

    // byte slice
    let byte_slice = &s[1..2]; // faster, but requires valid UTF-8 boundaries
    println!("\nbyte slice: {}", byte_slice);

    // convert to bytes (raw u8 access)
    let bytes: &[u8] = s.as_bytes();
    let first_char = bytes[0] as char; // only ASCII
    println!("\nfirst char: {}", first_char);

    // convert to Vec<char>
    let s_chars: Vec<char> = s.chars().collect();

    // use .chars().nth(idx)
    let first_char = s.chars().nth(1).unwrap();
    println!("\nfirst char: {}", first_char);

    // 4. find by val - find() vs. position()
    let s = "hello world";

    assert_eq!(s.find("world"), Some(6)); // substring found at byte index 6
    assert_eq!(s.find('o'), Some(4)); // first 'o' is at byte index 4
    assert_eq!(s.find("xyz"), None); // not found

    // position() on chars iterator
    // Search an iterator and
    // return the index of the first element where the predicate is true
    match s.chars().position(|c| c == 'w') {
        Some(idx) => println!("Position of 'w' (char index): {}", idx),
        None => println!("Not found"),
    }
    // 5. string-level check
    let s: String = String::from("hello world");
    let is_empty = s.is_empty();
    let char_len = s.chars().count();
    println!("char length: {}", char_len);
    let byte_len = s.as_bytes().len();
    println!("byte length: {}", byte_len);

    // substring in string
    // s.contains()
    // s.to_lowercase().contains(substring.to_lowercase())
    // s.find()

    // starts with or ends with
    // s.starts_with()
    // s.ends_with()

    // s.chars().all(|c| c.is_alphanumeric());
    // s.as_bytes().all(|b| b.is_ascii_alphanumeric());

    // 6. iterate
    let word = "hello";

    // iterate chars
    for c in word.chars() {
        // word.chars -> Iterator<Item =char>
        print!("{} ", c); // c is char bot &char
    }

    // iterate bytes
    for b in word.bytes() {
        // word.bytes -> Iterator<Item =u8>
        print!("{} ", b); // b is byte not &byte
    }

    // 6. Transform
    let sentence = "hello, world,  rust";
    // split by comma, trim, and join back
    let result = sentence
        .split(',')
        .map(|w| w.trim())
        .collect::<Vec<&str>>()
        .join(", ");
    println!("trimmed: {}", result);

    // capitalize first letter of each word
    let transformed: String = sentence
        .split_whitespace()
        .map(|word| {
            word.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().to_string()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ");

    println!("Capitalized: {}\n", transformed);

    // 8. String <-> Vec conversion

    // String -> Vec<char>
    let chars: Vec<char> = "hello".chars().collect();
    println!("String -> Vec<char>: {:?}", chars);

    // Vec<char> -> String
    let back: String = chars.iter().collect();
    println!("Vec<char> -> String: {}", back);

    // String -> Vec<u8>
    let bytes: Vec<u8> = "hello".bytes().collect();
    println!("String -> Vec<u8>: {:?}", bytes);

    // Vec<u8> -> String
    let from_bytes = String::from_utf8(bytes).unwrap();
    println!("Vec<u8> -> String: {}", from_bytes);

    // String -> Vec<&str> (split)
    let csv = "apple,banana,cherry";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("String -> Vec<&str>: {:?}", fruits);

    // Vec<&str> -> String
    let words: Vec<&str> = vec!["hello", "world", "rust"];
    let joined = words.join(" ");
    println!("Vec<&str> -> String (join): {}", joined);

    // Vec<String> -> String
    let string_vec: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    let result = string_vec.join(" - ");
    println!("Vec<String> -> String: {}", result);

    // Parse: String -> Vec<i32>
    let nums = "1,2,3,4,5";
    let parsed: Vec<i32> = nums.split(',').filter_map(|s| s.parse().ok()).collect();
    println!("Parse to Vec<i32>: {:?}", parsed);

    // Format: Vec<i32> -> String
    let formatted: String = parsed
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    println!("Format Vec<i32> -> String: {}", formatted);
}
