/**
 * 49. group anagrams
 *
 * Given an array of strings strs,
 * group the anagrams together.
 *
 * sort_unstable(): Typically 20-50% faster
 * sort_unstable(): Uses O(1) auxiliary space
 * sort(): Uses O(n) auxiliary space
 */

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // loop through strs
        // for each str, sort and insert into a hashmap, key is the original str
        // result is to convert hashmap.values into a vec of vec of strings.
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            // Sort the string to use as key
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key: String = chars.iter().collect();

            // Group by sorted key
            map.entry(key).or_insert(Vec::new()).push(s);
        }

        // Convert HashMap values to Vec<Vec<String>>
        map.into_values().collect()
    }
}

fn main() {
    let strs: Vec<String> = vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ];

    let result: Solution::group_anagrams(strs);

    println!("{;?}", result);
}
