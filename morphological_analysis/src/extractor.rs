use super::double_array::DoubleArray;
use super::double_array::INDEX_ROOT;

#[derive(Debug)]
pub struct Token {
    pub id: Option<usize>,
    pub start: usize,
    pub len: usize,
}
#[derive(Debug)]
pub struct ExtractResult {
    pub tokens: Vec<Token>,
}

pub fn extract(text: &str, da: &DoubleArray) -> ExtractResult {
    let mut tokens: Vec<Token> = vec![];
    let mut i: usize = 0;
    while i < text.chars().count() {
        let c = text.chars().nth(i).unwrap();
        if let Ok((mut cursor, _)) = da.transition(INDEX_ROOT, c) {
            let mut matched: Vec<Token> = vec![];
            match da.transition(cursor as usize, '\0') {
                Ok((_, stop_base)) => {
                    if stop_base < 0 {
                        matched.push(Token {
                            id: Some((stop_base * -1) as usize),
                            start: i,
                            len: i + 1,
                        });
                    }
                }
                Err(_) => {}
            }
            for (j, c) in text.chars().skip(i + 1).enumerate() {
                match da.transition(cursor as usize, c) {
                    Ok((next, next_base)) => {
                        if next_base < 0 {
                            matched.push(Token {
                                id: Some((next_base * -1) as usize),
                                start: i,
                                len: j + 1,
                            });
                        }
                        match da.transition(next as usize, '\0') {
                            Ok((_, stop_base)) => {
                                if stop_base < 0 {
                                    matched.push(Token {
                                        id: Some((stop_base * -1) as usize),
                                        start: i,
                                        len: j + 1,
                                    });
                                }
                            }
                            Err(_) => {}
                        }
                        cursor += next;
                    }
                    Err(_) => break,
                }
            }
            if let Some(longest) = matched.into_iter().max_by_key(|m| m.len) {
                i += longest.len - 1;
                tokens.push(longest);
            }
        }

        i += 1;
    }
    return ExtractResult { tokens };
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::set::IndexSet;

    fn uniondict() -> DoubleArray {
        // registered words: "a" and "bc"
        let mut codes = IndexSet::new();
        codes.insert('\0');
        codes.insert('a');
        codes.insert('b');
        codes.insert('c');
        let base: Vec<i32> = vec![0, 3, 0, -1, 3, 3, 7, -1];
        let check: Vec<usize> = vec![0, 0, 0, 4, 1, 1, 5, 6];
        return DoubleArray::from(base, check, codes);
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
