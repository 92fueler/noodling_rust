/**
 * https://exercism.org/tracks/rust/exercises/anagram
 *
* Given a target word and one or more candidate words, your task is to find the candidates that are anagrams of the target.
* An anagram is a rearrangement of letters to form a new word: for example "owns" is an anagram of "snow".
* A word is not its own anagram: for example, "stop" is not an anagram of "stop".
*
* The target word and candidate words are made up of one or more ASCII alphabetic characters (A-Z and a-z).
* Lowercase and uppercase characters are equivalent: for example, "PoTS" is an anagram of "sTOp", but "StoP" is not an anagram of "sTOp".
* The words you need to find should be taken from the candidate words, using the same letter case.
*
* Given the target "stone" and the candidate words "stone", "tones", "banana", "tons", "notes", and "Seton",
* the anagram words you need to find are "tones", "notes", and "Seton".
*
* The Rust track extends the possible letters to be any unicode character, not just ASCII alphabetic ones.
* You are going to have to adjust the function signature provided in the stub in order for the lifetimes to work out properly.
* This is intentional: what's there demonstrates the basics of lifetime syntax,
*  and what's missing teaches how to interpret lifetime-related compiler errors.
*
* Example:
* let word = "listen";
* let candidates = ["enlist", "google", "inlets", "banana"];
* let result = anagrams_for(word, &candidates);
*
*/
use std::collections::HashSet;

// lifetime meaning:
// The returned references come from possible_anagrams, not from word,
// and will never outlive the possible_anagrams slice.
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // Define a small helper closure that:
    // 1. lowercases the input word (case-insensitive comparison)
    // 2. converts it to chars
    // 3. sorts the chars to produce a normalized representation
    let normalize = |w: &str| -> Vec<char> {
        let mut chars: Vec<char> = w.to_lowercase().chars().collect();
        chars.sort_unstable(); // sort alphabetically (fast, stability doesn't matter)
        chars // last expression = returned value
    };

    // Normalize the target word so we can compare candidates against it
    let target_normalized = normalize(word);

    // Lowercase version used only to exclude identical words (e.g., "Listen" == "listen")
    let target_lower = word.to_lowercase();

    possible_anagrams
        .iter() // iterator over &&str (references to each candidate)
        .filter(|&&candidate| {
            let candidate_lower = candidate.to_lowercase();

            // Exclude the same word (case-insensitive)
            // Check if sorted characters match (true anagram test)
            candidate_lower != target_lower && normalize(candidate) == target_normalized
        })
        // Convert &&str → &str (copying a reference is trivial)
        .copied()
        // Collect matching candidates into a HashSet<&'a str>
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_anagrams_for() {
        let word = "listen";
        let candidates = ["enlist", "google", "inlets", "banana"];

        let result = anagrams_for(word, &candidates);

        // Build the expected HashSet<&str>
        let expected: HashSet<&str> = ["enlist", "inlets"]
            .iter() // Iterator<Item = &&str>
            .copied() // Iterator<Item = &str>
            .collect(); // collect into HashSet<&str>

        assert_eq!(result, expected);
    }
}
