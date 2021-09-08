pub mod double_array;

use double_array::DoubleArray;
use double_array::MatchResult;

#[derive(Debug)]
enum Token {
    Known { id: usize, start: usize, len: usize },
    Unknown { start: usize, len: usize },
}
#[derive(Debug)]
pub struct ExtractResult {
    text: String,
    tokens: Vec<Token>,
}

pub fn extract(text: &str, da: &DoubleArray) -> ExtractResult {
    let mut s = 1;
    let mut tokens: Vec<Token> = vec![];

    for (i, c) in text.chars().enumerate() {
        // Stop continue searching to start new matching
        if s != 1 && da.match_char(s, &c).is_none() {
            s = 1
        }

        match da.match_char(s, &c) {
            // The string up to this point is a word we know. But the match is not over yet.
            Some(MatchResult {
                next_state,
                can_stop,
            }) => {
                s = next_state;
                if can_stop {
                    tokens.push(Token::Known {
                        id: 1,    // FIXME
                        start: 1, // FIXME
                        len: i,   // FIXME
                    });
                    s = 1;
                }
            }
            None => {
                s = 1;
            }
        }
    }
    return ExtractResult {
        text: text.to_string(),
        tokens,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn uniondict() -> DoubleArray {
        // registered words: "a" and "bc"
        let mut codes: HashMap<char, usize> = HashMap::new();
        codes.insert('\0', 0);
        codes.insert('a', 1);
        codes.insert('b', 2);
        codes.insert('c', 3);
        let base: Vec<i32> = vec![0, 3, 0, -1, 3, 3, 7, -1];
        let check: Vec<usize> = vec![0, 0, 0, 4, 1, 1, 5, 6];
        return DoubleArray::new(base, check, codes);
    }

    #[test]
    fn uniondict_all_match() {
        let da = uniondict();
        assert_eq!(extract("a", &da).tokens.len(), 1);
        assert_eq!(extract("bc", &da).tokens.len(), 1);
    }

    #[test]
    fn uniondict_sequential_match() {
        let da = uniondict();
        assert_eq!(extract("aa", &da).tokens.len(), 2);
        assert_eq!(extract("abc", &da).tokens.len(), 2);
        assert_eq!(extract("bcbc", &da).tokens.len(), 2);
        assert_eq!(extract("bca", &da).tokens.len(), 2);
    }

    #[test]
    fn uniondict_starts_with_match() {
        let da = uniondict();
        assert_eq!(extract("ab", &da).tokens.len(), 1);
        assert_eq!(extract("bcc", &da).tokens.len(), 1);
    }

    #[test]
    fn uniondict_ends_with_match() {
        let da = uniondict();
        assert_eq!(extract("ba", &da).tokens.len(), 1);
        assert_eq!(extract("cbc", &da).tokens.len(), 1);
    }

    #[test]
    fn uniondict_starts_and_ends_with_match() {
        let da = uniondict();
        assert_eq!(extract("aba", &da).tokens.len(), 2);
        assert_eq!(extract("bcxbc", &da).tokens.len(), 2);
    }

    #[test]
    fn uniondict_partial_match_full_match() {
        let da = uniondict();
        assert_eq!(extract("ba", &da).tokens.len(), 1);
        assert_eq!(extract("bbc", &da).tokens.len(), 1);
    }

    #[test]
    fn uniondict_imcomplete_match() {
        let da = uniondict();
        assert_eq!(extract("b", &da).tokens.len(), 0);
        assert_eq!(extract("bbb", &da).tokens.len(), 0);
    }

    #[test]
    fn uniondict_unknown_chars() {
        let da = uniondict();
        assert_eq!(extract("x", &da).tokens.len(), 0);
        assert_eq!(extract("xax", &da).tokens.len(), 1);
        assert_eq!(extract("axa", &da).tokens.len(), 2);
    }
}
