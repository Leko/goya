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
    let chars: Vec<char> = text.chars().collect();
    while i < chars.len() {
        let c = chars[i];
        if let Ok((mut cursor, _)) = da.transition(INDEX_ROOT, c) {
            let mut matched: Vec<Token> = vec![];
            if let Ok(wid) = da.stop(cursor as usize) {
                matched.push(Token {
                    id: Some(wid),
                    start: i,
                    len: i + 1,
                });
            }
            let mut j = i + 1;
            while j < chars.len() {
                let c = chars[j];
                match da.transition(cursor as usize, c) {
                    Ok((next, _)) => {
                        if let Ok(wid) = da.stop(next as usize) {
                            matched.push(Token {
                                id: Some(wid),
                                start: i,
                                len: i + 1,
                            });
                        }
                        cursor = next;
                    }
                    Err(_) => {
                        break;
                    }
                }
                j += 1;
            }
            if let Some(longest) = matched.into_iter().max_by_key(|m| m.len) {
                // i += longest.len - 1;
                tokens.push(longest);
            }
        }

        i += 1;
    }
    ExtractResult { tokens }
}

#[cfg(test)]
mod tests {
    use super::super::common_prefix_tree::CommonPrefixTree;
    use super::*;
    use indexmap::IndexSet;

    fn dict1c1w() -> DoubleArray {
        let mut chars = IndexSet::new();
        chars.insert('\0');
        chars.insert('あ');
        // words: "あ"
        DoubleArray {
            codes: chars,
            base: vec![0, 1, -1],
            check: vec![0, 0, 1],
        }
    }

    fn dict2c1w() -> DoubleArray {
        let mut chars = IndexSet::new();
        chars.insert('\0');
        chars.insert('あ');
        // words: "ああ"
        DoubleArray {
            codes: chars,
            base: vec![0, 1, 2, -1],
            check: vec![0, 0, 1, 2],
        }
    }

    fn dict2c2w() -> DoubleArray {
        let mut chars = IndexSet::new();
        chars.insert('\0');
        chars.insert('あ');
        chars.insert('い');
        chars.insert('う');
        // words: "あい", "あう"
        DoubleArray {
            codes: chars,
            base: vec![0, 1, 1, -1, -2],
            check: vec![0, 0, 1, 2, 2],
        }
    }

    #[test]
    fn dict1c1w_just_the_string() {
        assert_eq!(extract("あ", &dict1c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict1c1w_starts_with() {
        assert_eq!(extract("あx", &dict1c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict1c1w_ends_with() {
        assert_eq!(extract("xあ", &dict1c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict1c1w_wrapped_with() {
        assert_eq!(extract("xあx", &dict1c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict1c1w_2char_sequence() {
        assert_eq!(extract("ああ", &dict1c1w()).tokens.len(), 2);
    }
    #[test]
    fn dict1c1w_wrapped_by() {
        assert_eq!(extract("あxあ", &dict1c1w()).tokens.len(), 2);
    }
    #[test]
    fn dict1c1w_2char_wrapped_by() {
        assert_eq!(extract("xあxあx", &dict1c1w()).tokens.len(), 2);
    }
    #[test]
    fn dict1c1w_3char_wrapped_by() {
        assert_eq!(extract("xあxあxあx", &dict1c1w()).tokens.len(), 3);
    }
    #[test]
    fn dict1c1w_3char_sequence() {
        assert_eq!(extract("あああ", &dict1c1w()).tokens.len(), 3);
    }
    #[test]
    fn dict1c1w_4char_wrapped_by() {
        assert_eq!(extract("xあxあxあxあx", &dict1c1w()).tokens.len(), 4);
    }
    #[test]
    fn dict1c1w_4char_sequence() {
        assert_eq!(extract("ああああ", &dict1c1w()).tokens.len(), 4);
    }
    #[test]
    fn dict1c1w_unknown_word() {
        assert_eq!(extract("x", &dict1c1w()).tokens.len(), 0);
    }
    #[test]
    fn dict1c1w_empty_string() {
        assert_eq!(extract("", &dict1c1w()).tokens.len(), 0);
    }

    #[test]
    fn dict2c1w_just_the_string() {
        assert_eq!(extract("ああ", &dict2c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c1w_starts_with() {
        assert_eq!(extract("ああx", &dict2c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c1w_ends_with() {
        assert_eq!(extract("xああ", &dict2c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c1w_wrapped_with() {
        assert_eq!(extract("xああx", &dict2c1w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c1w_2char_sequence() {
        // 3 = [0, 1], [1, 2], and [2, 3]
        assert_eq!(extract("ああああ", &dict2c1w()).tokens.len(), 3);
    }
    #[test]
    fn dict2c1w_wrapped_by() {
        assert_eq!(extract("ああxああ", &dict2c1w()).tokens.len(), 2);
    }
    #[test]
    fn dict2c1w_2char_wrapped_by() {
        assert_eq!(extract("xああxああx", &dict2c1w()).tokens.len(), 2);
    }
    #[test]
    fn dict2c1w_3char_wrapped_by() {
        assert_eq!(extract("xああxああxああx", &dict2c1w()).tokens.len(), 3);
    }
    #[test]
    fn dict2c1w_3char_sequence() {
        assert_eq!(extract("ああああああ", &dict2c1w()).tokens.len(), 5);
    }
    #[test]
    fn dict2c1w_4char_wrapped_by() {
        assert_eq!(
            extract("xああxああxああxああx", &dict2c1w()).tokens.len(),
            4
        );
    }
    #[test]
    fn dict2c1w_4char_sequence() {
        assert_eq!(extract("ああああああああ", &dict2c1w()).tokens.len(), 7);
    }
    #[test]
    fn dict2c1w_unknown_word() {
        assert_eq!(extract("x", &dict2c1w()).tokens.len(), 0);
    }
    #[test]
    fn dict2c1w_empty_string() {
        assert_eq!(extract("", &dict2c1w()).tokens.len(), 0);
    }

    #[test]
    fn dict2c2w_just_the_string() {
        assert_eq!(extract("あい", &dict2c2w()).tokens.len(), 1);
        assert_eq!(extract("あう", &dict2c2w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c2w_starts_with() {
        assert_eq!(extract("あいx", &dict2c2w()).tokens.len(), 1);
        assert_eq!(extract("あうx", &dict2c2w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c2w_ends_with() {
        assert_eq!(extract("xあい", &dict2c2w()).tokens.len(), 1);
        assert_eq!(extract("xあう", &dict2c2w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c2w_wrapped_with() {
        assert_eq!(extract("xあいx", &dict2c2w()).tokens.len(), 1);
        assert_eq!(extract("xあうx", &dict2c2w()).tokens.len(), 1);
    }
    #[test]
    fn dict2c2w_2char_sequence() {
        assert_eq!(extract("あいあう", &dict2c2w()).tokens.len(), 2);
        assert_eq!(extract("あいあい", &dict2c2w()).tokens.len(), 2);
        assert_eq!(extract("あうあい", &dict2c2w()).tokens.len(), 2);
    }
    #[test]
    fn dict2c2w_wrapped_by() {
        assert_eq!(extract("あいxあい", &dict2c2w()).tokens.len(), 2);
        assert_eq!(extract("あいxあう", &dict2c2w()).tokens.len(), 2);
        assert_eq!(extract("あうxあう", &dict2c2w()).tokens.len(), 2);
    }

    #[test]
    fn test_dict2c2w() {
        let mut chars = IndexSet::new();
        chars.insert('\0');
        chars.insert('う');
        chars.insert('ん');
        chars.insert('と');

        let da = DoubleArray {
            codes: chars,
            base: vec![0, 1, 1, 1, -1],
            check: vec![0, 0, 1, 2, 3],
        };
        assert_eq!(extract("うん", &da).tokens.len(), 0);
        assert_eq!(extract("うんと", &da).tokens[0].id, Some(1));
        assert_eq!(extract("うーんと", &da).tokens.len(), 0);
        assert_eq!(extract("うんとこどっこいしょ", &da).tokens[0].id, Some(1));
    }

    #[test]
    fn test_ipadic2() {
        let mut chars = IndexSet::new();
        chars.insert('\0');
        chars.insert('あ');
        chars.insert('ー');

        let da = DoubleArray {
            codes: chars,
            base: vec![0, 1, 3, -1, 0, -2],
            check: vec![0, 0, 1, 2, 0, 2],
        };

        assert_eq!(extract("あ", &da).tokens[0].id, Some(1));
        assert_eq!(extract("あー", &da).tokens[0].id, Some(2));
    }

    #[test]
    fn test_dict3c2w() {
        let mut chars = IndexSet::new();
        chars.insert('\0');
        chars.insert('う');
        chars.insert('ん');
        chars.insert('と');
        chars.insert('え');

        let mut trie = CommonPrefixTree::new();
        trie.append(1, "うんと");
        trie.append(2, "ええと");
        let da = DoubleArray::from_trie(&trie, |(_, _)| {});

        assert_eq!(extract("うん", &da).tokens.len(), 0);
        assert_eq!(extract("ええ", &da).tokens.len(), 0);
        assert_eq!(extract("うんと", &da).tokens[0].id, Some(1));
        assert_eq!(extract("ええと", &da).tokens[0].id, Some(2));
        assert_eq!(extract("うんとこどっこいしょ", &da).tokens[0].id, Some(1));
    }

    #[test]
    fn test_a_and_bc() {
        let mut trie = CommonPrefixTree::new();
        trie.append(1, "a");
        trie.append(2, "bc");
        let da = DoubleArray::from_trie(&trie, |(_, _)| {});

        assert_eq!(extract("a", &da).tokens[0].id, Some(1));
        assert_eq!(extract("bc", &da).tokens[0].id, Some(2));
    }
}
