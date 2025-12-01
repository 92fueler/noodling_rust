/**
14. Longest Common Prefix

find the longest common prefix amongst an array of strings.
*/

fn longest_common_prefix(strs: Vec<String>) -> String {}

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
