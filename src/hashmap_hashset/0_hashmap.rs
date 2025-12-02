use std::collections::HashMap;

fn main() {
    // create an empty
    let mut hash_map: HashMap<String, i32> = HashMap::new();

    // create from array of tuples
    let mut hash_map: HashMap<&str, i32> =
        HashMap::from([("apple", 3), ("banana", 5), ("orange", 2)]);

    // Using collect() from iterator
    let mut hash_map: HashMap<_, _> = [("a", 1), ("b", 2)].iter().cloned().collect();

    // Basic insert - returns Option<V> (old value if key existed)
    hash_map.insert("key", 42);
    let returned = hash_map.insert("key", 100).unwrap(); // overwrites, returns Some(42)
    println!("{}", returned); // 42

    println!("{:?}", hash_map);

    // Insert only if key doesn't exist
    hash_map.entry("key").or_insert(50); // won't overwrite

    // Get mutable reference or insert default
    let value = hash_map.entry("new_key").or_insert(0);
    *value += 10; // modify in place

    // Update based on old value
    hash_map
        .entry("counter")
        .and_modify(|v| *v += 1)
        .or_insert(1);
    println!("{:?}", hash_map);
    // --------------------------------------------------------------------
    let mut map = HashMap::from([("a", 1), ("b", 2)]);

    // Check if key exists
    if map.contains_key("a") {
        println!("Found!");
    }

    // Remove - returns Option<V>
    if let Some(value) = map.remove("a") {
        println!("Removed: {}", value);
    }

    // Get size
    println!("Length: {}", map.len());

    // Check if empty
    if map.is_empty() {
        println!("Empty map");
    }

    // Clear all entries
    map.clear();
}
