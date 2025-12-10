use std::collections::HashMap;

/**
 *  1. create a hashmap (3)
 *  2. get hashmap properties: is_empty(), len()
 *  3. operations:
 *      - insert value by key
 *          - basic insert, return Option<V> for old value if key exists
 *          - insert only if key doesn't exist
 *          - insert and update the value
 *      - check if a key exists
 *      - remove a key-value pair
 *      - clear the entire hashmap
 *      - loop through the key-value pairs
 *  4. common patterns:
 *      - Given a string, create a char freq
 */

fn main() {
    // 1. create a hashmap
    // create an empty hashmap
    let mut hash_map: HashMap<String, i32> = HashMap::new();

    // create from array of tuples
    let mut hash_map: HashMap<&str, i32> =
        HashMap::from([("apple", 3), ("banana", 5), ("orange", 2)]);

    // Using collect() from iterator
    let mut hash_map: HashMap<_, _> = [("a", 1), ("b", 2)].iter().cloned().collect();

    // 2. operations
    // Basic insert - returns Option<V> (old value if key existed)
    hash_map.insert("key", 42);
    let returned = hash_map.insert("key", 100).unwrap(); // overwrites, returns Some(42)
    println!("{}", returned); // 42

    println!("{:?}", hash_map);

    // Insert only if key doesn't exist
    hash_map.entry("key").or_insert(50); // won't overwrite

    // Get mutable reference or insert default
    let value = hash_map.entry("key").or_insert(0);
    *value += 10; // modify in place
    println!("from last insert: {:?}", hash_map);

    let mut map = HashMap::from([("a", 1), ("b", 2)]);

    // Check if key exists
    if map.contains_key("a") {
        println!("Found!");
    }

    if let Some(value) = map.remove("a") {
        // Remove - returns Option<V>
        println!("Removed: {}", value);
    }
    map.clear();

    // 3. loop through
    for (k, v) in &hash_map {
        println!("{}: {}", k, v);
    }

    for (k, v) in &mut hash_map {
        *v += 100;
        println!("{}: {}", k, v);
    }

    for k in hash_map.keys() {
        println!("key: {}", k);
    }

    for k in hash_map.values() {
        println!("value: {}", k);
    }

    // 4. Common pattern
    // char freq
    fn char_freq(s: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();

        for c in s.chars() {
            // entry(c) gets the entry for key `c`
            // or_insert(0) gives a &mut usize, starting from 0 if missing
            *freq.entry(c).or_insert(0) += 1;
        }

        freq
    }

    // char freq with cleanup
    fn char_freq_letters_only(s: &str) -> HashMap<char, usize> {
        let mut freq = HashMap::new();

        for c in s.chars() {
            if c.is_whitespace() {
                continue;
            }

            let c = c.to_ascii_lowercase(); // normalize case

            *freq.entry(c).or_insert(0) += 1;
        }

        freq
    }
}
