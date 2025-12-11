/**
 * 271. encode and decode strings
 *
 * Design an algorithm to encode a list of strings to a string.
 * The encoded string is then sent over the network
 * and is decoded back to the original list of strings.
 */
struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            // Format: "length:string"
            result.push_str(&format!("{}:{}", s.len(), s));
        }

        result
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes();

        while i < bytes.len() {
            // Find the colon
            let mut j = i;
            while j < bytes.len() && bytes[j] != b':' {
                j += 1;
            }

            // Parse length
            let len: usize = s[i..j].parse().unwrap();
            i = j + 1; // Skip the colon

            // Extract the string
            result.push(s[i..i + len].to_string());
            i += len;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let codec = Codec::new();
        let strs = vec!["hello".to_string(), "world".to_string()];
        let encoded = codec.encode(strs.clone());
        assert_eq!(codec.decode(encoded), strs);
    }

    #[test]
    fn test_with_special_chars() {
        let codec = Codec::new();
        let strs = vec!["a:b".to_string(), "c,d".to_string()];
        let encoded = codec.encode(strs.clone());
        assert_eq!(codec.decode(encoded), strs);
    }

    #[test]
    fn test_empty_strings() {
        let codec = Codec::new();
        let strs = vec!["".to_string(), "a".to_string(), "".to_string()];
        let encoded = codec.encode(strs.clone());
        assert_eq!(codec.decode(encoded), strs);
    }

    #[test]
    fn test_single_string() {
        let codec = Codec::new();
        let strs = vec!["hello".to_string()];
        let encoded = codec.encode(strs.clone());
        assert_eq!(codec.decode(encoded), strs);
    }
}
