/**
 * 604. design compressed string iterator
 *
 * Convert a compressed string like "L1e2t1" into an iterator that returns characters one by one: 'L', 'e', 'e', 't'.
 *
 * Input: "L1e2t1C1o1d1e1"
 * Output: next() -> 'L', next() -> 'e', next() -> 'e', next() -> 't', ...
 */
struct StringIterator {
    chars: Vec<char>,
    counts: Vec<usize>,
    char_idx: usize,   // Current character index
    count_left: usize, // How many of current char left to return
}

impl StringIterator {
    fn new(compressed_string: String) -> Self {
        let mut chars = Vec::new();
        let mut counts = Vec::new();
        let bytes = compressed_string.as_bytes();
        let mut i = 0;

        while i < bytes.len() {
            // Read character
            let ch = bytes[i] as char;
            chars.push(ch);
            i += 1;

            // Read count (could be multiple digits)
            let mut count = 0;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                count = count * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            counts.push(count);
        }

        let count_left = if !counts.is_empty() { counts[0] } else { 0 };

        StringIterator {
            chars,
            counts,
            char_idx: 0,
            count_left,
        }
    }

    fn next(&mut self) -> char {
        if !self.has_next() {
            return ' '; // Or any sentinel value
        }

        let result = self.chars[self.char_idx];
        self.count_left -= 1;

        // Move to next character if current is exhausted
        if self.count_left == 0 && self.char_idx + 1 < self.chars.len() {
            self.char_idx += 1;
            self.count_left = self.counts[self.char_idx];
        }

        result
    }

    fn has_next(&self) -> bool {
        self.char_idx < self.chars.len() && self.count_left > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut iter = StringIterator::new("L1e2t1C1o1d1e1".to_string());
        assert_eq!(iter.next(), 'L');
        assert_eq!(iter.next(), 'e');
        assert_eq!(iter.next(), 'e');
        assert_eq!(iter.next(), 't');
        assert_eq!(iter.next(), 'C');
        assert_eq!(iter.next(), 'o');
        assert_eq!(iter.next(), 'd');
        assert_eq!(iter.next(), 'e');
        assert!(!iter.has_next());
    }

    #[test]
    fn test_has_next() {
        let mut iter = StringIterator::new("a2".to_string());
        assert!(iter.has_next());
        iter.next();
        assert!(iter.has_next());
        iter.next();
        assert!(!iter.has_next());
    }

    #[test]
    fn test_multi_digit_count() {
        let mut iter = StringIterator::new("a10".to_string());
        for _ in 0..10 {
            assert_eq!(iter.next(), 'a');
        }
        assert!(!iter.has_next());
    }

    #[test]
    fn test_single_char() {
        let mut iter = StringIterator::new("x5".to_string());
        assert_eq!(iter.next(), 'x');
        assert_eq!(iter.next(), 'x');
        assert_eq!(iter.next(), 'x');
        assert_eq!(iter.next(), 'x');
        assert_eq!(iter.next(), 'x');
        assert!(!iter.has_next());
    }

    #[test]
    fn test_empty_after_exhausted() {
        let mut iter = StringIterator::new("a1".to_string());
        iter.next();
        assert_eq!(iter.next(), ' '); // Sentinel value when exhausted
    }
}
