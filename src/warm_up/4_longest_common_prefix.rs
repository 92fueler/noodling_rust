/**
 * 14. Longest Common Prefix
 * find the longest common prefix amongst an array of strings.
*/

fn longest_common_prefix(strs: Vec<String>) -> String {
    // strs[0] is a String
    // .chars() returns a Chars iterator over char values
    // .collect() gathers them into Vec<char>
    let first_str_chars: Vec<char> = strs[0].chars().collect();

    let mut result: String = String::new();

    // .iter() borrows the Vec<char>, creating Iterator<Item = &char>
    // .enumerate() wraps it: Iterator<Item = (usize, &char)>
    for (idx, c) in first_str_chars.iter().enumerate() {
        // idx: usize
        // c: &char (reference to a char in the vector)

        // strs.iter() creates Iterator<Item = &String>
        // .skip(1) still gives Iterator<Item = &String>
        for w in strs.iter().skip(1) {
            // w: &String (borrowed reference to each String in strs)

            // w.chars() returns Chars iterator
            // .collect() creates a new Vec<char>
            let w_chars: Vec<char> = w.chars().collect();

            // w_chars[idx] gives char (owned value from the vector)
            // *c dereferences &char to get char (for comparison)
            if idx >= w_chars.len() || w_chars[idx] != *c {
                return result;
            }
            // *c dereferences &char to char to push into String
            result.push(*c);
        }
    }

    result
}

fn main() {
    let strs: Vec<String> = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
        String::from("f"),
    ];

    let result = longest_common_prefix(strs);
    println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(longest_common_prefix(strs), "fl");
    }
}
