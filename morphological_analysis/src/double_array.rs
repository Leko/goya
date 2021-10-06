use super::common_prefix_tree::CommonPrefixTree;
use core::panic;
use indexmap::IndexSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;

const INDEX_ROOT: usize = 1;
const TERM_CHAR: char = '\0';

#[derive(Debug)]
pub enum TransitionError {
    AlreadyTerminated,
    BaseFailed,
    CheckFailed,
    UnknownChar,
    BaseOutOfBounds,
    CheckOutOfBounds,
}

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct DoubleArray {
    pub codes: IndexSet<char>,
    pub base: Vec<i32>,
    pub check: Vec<usize>,
}
impl Default for DoubleArray {
    fn default() -> Self {
        let base: Vec<i32> = vec![0, 1];
        let check: Vec<usize> = vec![0, 0];
        let mut codes: IndexSet<char> = IndexSet::new();

        codes.insert(TERM_CHAR);

        DoubleArray { base, check, codes }
    }
}
impl DoubleArray {
    pub fn from(base: Vec<i32>, check: Vec<usize>, codes: IndexSet<char>) -> Self {
        DoubleArray { base, check, codes }
    }

    pub fn wids(&self) -> impl Iterator<Item = usize> + '_ {
        self.base
            .iter()
            .filter(|s| **s < 0)
            .map(|s| as_usize(&(s * -1)))
    }

    pub fn get_exact(&self, term: &str) -> Option<usize> {
        let mut s = INDEX_ROOT;
        for c in term.chars() {
            match self.transition(s, c) {
                Ok((next, _)) => s = next as usize,
                _ => return None,
            }
        }
        match self.stop(s) {
            Ok(id) => Some(id),
            _ => None,
        }
    }

    pub fn from_cpt(trie: &CommonPrefixTree) -> Self {
        let mut state_cache = HashMap::new();
        let mut da = DoubleArray::default();
        let mut chars = trie
            .entires_dfs()
            .iter()
            .map(|(prefix, _)| prefix)
            .join("")
            .chars()
            .collect::<Vec<_>>();
        chars.sort_unstable();
        chars.dedup();
        for c in chars {
            da.insert_to_codes(c);
        }

        for (prefix, node) in trie.entires_dfs() {
            if node.can_stop() {
                continue;
            }

            // root node
            if prefix.is_empty() {
                for next_c in node.children.keys() {
                    let next_char_code = da.get_code(next_c).unwrap();
                    let t = da.base[INDEX_ROOT] + next_char_code as i32;
                    let t = as_usize(&t);
                    da.insert_to_check(t, INDEX_ROOT);
                    state_cache.insert(concat_char_to_str(&prefix, *next_c), t);
                }
                continue;
            }

            let s = *state_cache
                .get(&prefix)
                .unwrap_or_else(|| panic!("Unknown prefix: {:?}", prefix));
            da.insert_to_base(s, da.find_next_s(node));
            for (next_c, child) in node.children.iter() {
                let t = da.base.get(s).unwrap() + da.get_code(next_c).unwrap() as i32;
                let t = as_usize(&t);
                da.insert_to_check(t, s);
                if child.can_stop() {
                    da.insert_to_base(t, -(child.id.unwrap() as i32));
                } else {
                    let key = concat_char_to_str(&prefix, *next_c);
                    state_cache.insert(key, t);
                }
            }
        }
        da.base.shrink_to_fit();
        da.check.shrink_to_fit();
        da.codes.shrink_to_fit();
        da
    }

    pub fn transition(
        &self,
        from: usize,
        to: char,
    ) -> Result<(i32, Option<usize>), TransitionError> {
        let code = self.get_code(&to).ok_or(TransitionError::UnknownChar)?;
        let s = self
            .base
            .get(from)
            .ok_or(TransitionError::BaseOutOfBounds)?;
        let t = s + code as i32;
        if t < 0 {
            return Err(TransitionError::AlreadyTerminated);
        }
        let next = self
            .check
            .get(as_usize(&t))
            .ok_or(TransitionError::CheckOutOfBounds)?;
        let base = self
            .base
            .get(t as usize)
            .ok_or(TransitionError::BaseFailed)?;
        let wid = if *base < 0 {
            Some(as_usize(&(base * -1)))
        } else {
            None
        };
        if *next == from {
            Ok((t, wid))
        } else {
            Err(TransitionError::CheckFailed)
        }
    }

    pub fn init(&self, to: char) -> Result<(i32, Option<usize>), TransitionError> {
        self.transition(INDEX_ROOT, to)
    }

    pub fn stop(&self, from: usize) -> Result<usize, TransitionError> {
        match self.transition(from, TERM_CHAR) {
            Ok((_, Some(wid))) => Ok(wid),
            Ok(_) => unreachable!("Successful transition, but no wid"),
            Err(reason) => Err(reason),
        }
    }

    pub fn get_code(&self, c: &char) -> Option<usize> {
        self.codes.get_full(c).map(|(code, _)| code)
    }

    fn insert_to_codes(&mut self, c: char) -> usize {
        let (char_code, _) = self.codes.insert_full(c);
        char_code
    }

    fn insert_to_base(&mut self, index: usize, value: i32) {
        let resized = cmp::max(self.base.len(), index + 1);
        self.base.resize(resized, 0);
        assert_eq!(
            self.base[index], 0,
            "index={} already used: {:?}",
            index, self.base
        );
        self.base[index] = value;
    }

    fn insert_to_check(&mut self, index: usize, value: usize) {
        let resized = cmp::max(self.check.len(), index + 1);
        self.check.resize(resized, 0);
        self.check[index] = value;
    }

    fn get_available_check_index(&self, left: usize) -> usize {
        self.check
            .iter()
            .enumerate()
            .skip(left)
            // clippy says that `find is prefered to skip_while+next` but it's slower than the current
            .skip_while(|(_, value)| value != &&0)
            .next()
            .map(|(i, _)| i)
            .unwrap_or_else(|| unreachable!("index must be found"))
    }

    fn find_next_s(&self, child: &CommonPrefixTree) -> i32 {
        let mut position = self.get_available_check_index(INDEX_ROOT + 1);
        let min_code = self.get_code(child.min_char().unwrap()).unwrap();
        let offsets: Vec<_> = child
            .children
            .keys()
            .map(|c| self.get_code(c).unwrap() - min_code)
            .collect();
        while offsets
            .iter()
            .any(|code| match self.check.get(position + code) {
                Some(0) => false,
                Some(_) => true,
                _ => false,
            })
        {
            position += 1;
        }
        (position - min_code) as i32
    }
}

fn as_usize(n: &i32) -> usize {
    assert!(*n >= 0, "n({}) should be greater than or equal to 0", n);
    *n as usize
}

fn concat_char_to_str(text: &str, c: char) -> String {
    let mut tmp = String::from(text);
    tmp.push(c);
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_a_word_that_has_1_char() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('あ');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "あ");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, -1]);
        assert_eq!(da.check, vec![0, 0, 1]);
    }

    #[test]
    fn builds_word_with_the_same_letter_in_succession() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('あ');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "ああ");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, 2, -1]);
        assert_eq!(da.check, vec![0, 0, 1, 2]);
    }

    #[test]
    fn builds_two_words_that_have_1_char() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('あ');
        chars.insert('い');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "あ");
        trie.append(2, "い");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, -1, -2]);
        assert_eq!(da.check, vec![0, 0, 1, 1]);
    }

    #[test]
    fn builds_common_prefixed_words() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('あ');
        chars.insert('い');
        chars.insert('う');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "あい");
        trie.append(2, "あう");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, 1, -1, -2]);
        assert_eq!(da.check, vec![0, 0, 1, 2, 2]);
    }

    #[test]
    fn builds_intersect_words() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('あ');
        chars.insert('い');
        chars.insert('う');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "あい");
        trie.append(2, "いう");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, 2, 2, -1, -2]);
        assert_eq!(da.check, vec![0, 0, 1, 1, 2, 3]);
    }

    #[test]
    fn builds_3chars_word() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('う');
        chars.insert('ん');
        chars.insert('と');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "うんと");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, 0, 2, -1]);
        assert_eq!(da.check, vec![0, 0, 1, 2, 3]);
    }

    #[test]
    fn builds_ipadic2() {
        let mut chars = IndexSet::new();
        chars.insert(TERM_CHAR);
        chars.insert('あ');
        chars.insert('ー');

        let mut trie = CommonPrefixTree::default();
        trie.append(1, "あ");
        trie.append(2, "あー");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.codes, chars);
        assert_eq!(da.base, vec![0, 1, 3, -1, 0, -2]);
        assert_eq!(da.check, vec![0, 0, 1, 2, 0, 2]);
    }

    #[test]
    fn test_get_exact() {
        let mut trie = CommonPrefixTree::default();
        trie.append(1, "a");
        trie.append(2, "aa");
        trie.append(3, "aaa");
        trie.append(4, "aaaa");
        trie.append(5, "aaaaa");
        trie.append(6, "ab");
        trie.append(7, "abc");
        trie.append(8, "abcd");
        trie.append(9, "abcde");
        trie.append(10, "abcdef");
        let da = DoubleArray::from_cpt(&trie);

        assert_eq!(da.get_exact("a"), Some(1));
        assert_eq!(da.get_exact("aa"), Some(2));
        assert_eq!(da.get_exact("aaa"), Some(3));
        assert_eq!(da.get_exact("aaaa"), Some(4));
        assert_eq!(da.get_exact("aaaaa"), Some(5));
        assert_eq!(da.get_exact("ab"), Some(6));
        assert_eq!(da.get_exact("abc"), Some(7));
        assert_eq!(da.get_exact("abcd"), Some(8));
        assert_eq!(da.get_exact("abcde"), Some(9));
        assert_eq!(da.get_exact("abcdef"), Some(10));
    }
}
