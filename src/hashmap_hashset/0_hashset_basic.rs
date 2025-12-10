use std::collections::HashSet;

/**
 * 1. create a hashset (3)
 * 2. hashset properties: is_empty(), len()
 * 3. operations:
 *      - insert value, returns true/false
 *      - check if a value exists
 *      - remove a value
 *      - clear the entire set
 *      - loop through elements: don't support mutable iteration
 *
 * 4. Common patterns:
 *      - Given a string, get unique chars
 */

fn main() {
    // 1. create a hashset
    let mut set: HashSet<String> = HashSet::new();

    // create from array or slice using from()
    let mut set: HashSet<&str> = HashSet::from(["apple", "banana", "pearl"]);

    // using collect() from iterator
    let mut set: HashSet<_> = ["a", "b", "c"].iter().cloned().collect();

    // 2. properties
    if set.is_empty() {
        println!("empty set");
    }

    let set_len = set.len();
    println!("length: {}", set_len);

    // 3. operations

    // Insert a value
    let inserted = set.insert("pear"); // returns true if value was newly inserted,
    println!("Inserted pear? {}", inserted);
    let inserted_again = set.insert("pear"); // false if value was already in the set
    println!("Inserted pear again? {}", inserted_again); // false

    // Check if a value exists
    if set.contains("banana") {
        println!("banana is in the set");
    }

    // Remove a value
    if set.remove("banana") {
        println!("banana removed");
    }

    // Clear the entire set
    set.clear();
    println!("after clear, is empty? {}", set.is_empty());

    // 3. loop
    let mut set = HashSet::from(["x", "y", "z"]);

    for value in &set {
        println!("value: {}", value);
    }

    // HashSet does NOT support mutable iteration (values are unique),
    // you cannot do &mut because there’s no "value" to mutate—only identities.

    // 4. common patterns:
    // basic unique chars
    fn unique_chars(s: &str) -> HashSet<char> {
        let mut set = HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        set
    }

    // unique letters only (lowercase normalize)
    fn unique_letters_only(s: &str) -> HashSet<char> {
        let mut set = HashSet::new();
        for c in s.chars() {
            if c.is_whitespace() {
                continue;
            }
            set.insert(c.to_ascii_lowercase());
        }
        set
    }
}
