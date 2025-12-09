fn main() {
    println!("=== STRING CREATION (5 ways) ===");
    let s1: &str = "hello"; // string literal (immutable, stack/static)
    let s2: String = "hello".to_string(); // &str -> String (heap allocated)
    let s3: String = String::from("hello"); // same as to_string()
    let s4: String = String::new(); // empty String
    let s5: &str = &s3; // String -> &str (borrowing)

    println!("=== CONCATENATION (3 ways) ===");
    let mut s: String = String::from("hello");
    s += " world"; // append &str
    s.push('!'); // append single char
    s.push_str(" Jian"); // append &str
    println!("Concatenated: {}\n", s);

    println!("=== FORMAT! MACRO ===");
    let word1 = "hello";
    let word2 = "world";
    let result: String = format!("{} {}!", word1, word2);
    println!("Formatted: {}\n", result);

    println!("=== INSERT vs INSERT_STR ===");
    s.insert(10, '?'); // insert char at index (byte position)
    println!("After insert('?'): {}", s);

    s.insert_str(11, "[STR]"); // insert &str at index
    println!("After insert_str: {}\n", s);

    println!("=== POP (handle Option<char> (4 ways) ===");
    // 1. if let - cleanest for simple cases
    if let Some(c) = s.pop() {
        println!("Popped char: {:?}", c);
        println!("As string: {}", c.to_string());
    }
    println!("After pop: {}\n", s);

    // 2. unwrap_or - provide default value
    let mut s_test = String::from("test");
    let c = s_test.pop().unwrap_or('_');
    println!("Popped or default '_': {}", c);

    // 3. unwrap_or_else - lazy default (closure only runs if None)
    let mut s_empty = String::new();
    let c = s_empty.pop().unwrap_or_else(|| {
        println!("Empty string, using default!");
        '?'
    });
    println!("Result: {}\n", c);

    // 4. unwrap - panics if None (use when certain it's not empty)
    let mut s_certain = String::from("x");
    let c = s_certain.pop().unwrap();
    println!("Unwrapped: {}\n", c);

    println!("=== REMOVE (returns char) ===");
    let removed_char = s.remove(10); // removes char at byte index, returns it
    println!("Removed char at index 10: {:?}", removed_char);
    println!("After remove: {}\n", s);

    println!("=== POSITION & FIND ===");
    let text = "hello world";

    // find() returns Option<usize> (byte index)
    match text.find('w') {
        Some(idx) => println!("Found 'w' at byte index: {}", idx),
        None => println!("Not found"),
    }

    match text.find("world") {
        Some(idx) => println!("Found 'world' at byte index: {}", idx),
        None => println!("Not found"),
    }

    // position() on chars iterator
    match text.chars().position(|c| c == 'w') {
        Some(idx) => println!("Position of 'w' (char index): {}", idx),
        None => println!("Not found"),
    }
    println!();

    println!("=== ITERATE ===");
    let word = "hello";

    // iterate chars
    print!("Chars: ");
    for c in word.chars() {
        print!("{} ", c);
    }
    println!();

    // iterate bytes
    print!("Bytes: ");
    for b in word.bytes() {
        print!("{} ", b);
    }
    println!("\n");

    println!("=== ITERATE & TRANSFORM ===");
    let text = "hello";

    // to uppercase
    let upper: String = text.chars().map(|c| c.to_uppercase().to_string()).collect();
    println!("Uppercase: {}", upper);

    // reverse
    let reversed: String = text.chars().rev().collect();
    println!("Reversed: {}\n", reversed);

    println!("=== SPLIT, TRANSFORM, REJOIN ===");
    let sentence = "hello world rust";

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

    println!("Original: {}", sentence);
    println!("Capitalized: {}\n", transformed);

    println!("=== STRING <-> VEC CONVERSIONS ===");

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
