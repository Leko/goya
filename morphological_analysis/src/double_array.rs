use super::trie_tree::TrieTree;
use indexmap::IndexSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;

pub const INDEX_ROOT: usize = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct DoubleArray {
    pub codes: IndexSet<char>,
    pub base: Vec<i32>,
    pub check: Vec<usize>,
}
impl DoubleArray {
    pub fn new() -> DoubleArray {
        let base: Vec<i32> = vec![0, 1];
        let check: Vec<usize> = vec![0, 0];
        let mut codes: IndexSet<char> = IndexSet::new();

        codes.insert('\0');

        DoubleArray { base, check, codes }
    }

    pub fn from(base: Vec<i32>, check: Vec<usize>, codes: IndexSet<char>) -> DoubleArray {
        DoubleArray { base, check, codes }
    }

    pub fn from_trie(trie: &TrieTree, f: impl Fn((usize, usize))) -> DoubleArray {
        let mut state_cache = HashMap::new();
        let mut da = DoubleArray::new();
        let mut completed = 0;
        let total = trie.size();
        for (prefix, node) in trie.entires() {
            f((completed, total));
            completed += 1;
            // root node
            if prefix.is_empty() {
                for next_c in node.children.keys().sorted() {
                    let next_char_code = da.insert_to_codes(*next_c);
                    let t = da.base[INDEX_ROOT] + next_char_code as i32;
                    let t = as_usize(&t);
                    da.insert_to_check(t, INDEX_ROOT);
                    state_cache.insert(concat_char_to_str(&prefix, *next_c), t);
                }
                continue;
            }

            let c = prefix.chars().last().unwrap_or('\0');
            // FIXME: Is doing it here the best?
            // We might have calculate it before appending to base
            da.insert_to_codes(c);

            let s = *state_cache
                .get(&prefix)
                .unwrap_or_else(|| panic!("Unknown prefix: {:?}", prefix));
            if node.can_stop() {
                da.insert_to_base(s, node.id.unwrap() as i32 * -1);
                continue;
            }
            for next_c in node.children.keys().sorted() {
                da.insert_to_codes(*next_c);
            }
            da.insert_to_base(s, da.find_s(s, &node.children));
            for next_c in node.children.keys().sorted() {
                let t = da.base.get(s).unwrap() + da.get_code(next_c).unwrap() as i32;
                let t = as_usize(&t);
                da.insert_to_check(t, s);
                let key = concat_char_to_str(&prefix, *next_c);
                state_cache.insert(key, t);
            }
        }
        da.base.shrink_to_fit();
        da.check.shrink_to_fit();
        da
    }

    pub fn transition(&self, from: usize, to: char) -> Result<(i32, i32), &str> {
        match self.get_code(&to) {
            Some(code) => {
                let t = self.base.get(from).unwrap() + code as i32;
                if t < 0 {
                    return Err("already reached the end character");
                }
                if *self.check.get(t as usize).unwrap() == from {
                    match self.base.get(t as usize) {
                        Some(base) => Ok((t, *base)),
                        None => Err("failed to fetch base"),
                    }
                } else {
                    Err("failed to check")
                }
            }
            None => Err("unknown char"),
        }
    }

    pub fn get_code(&self, c: &char) -> Option<usize> {
        match self.codes.get_full(c) {
            Some((code, _)) => Some(code),
            None => None,
        }
    }

    fn insert_to_codes(&mut self, c: char) -> usize {
        let (char_code, _) = self.codes.insert_full(c);
        char_code
    }

    fn insert_to_base(&mut self, index: usize, value: i32) {
        let resized = cmp::max(self.base.len(), index);
        self.base.resize(resized, 0);
        self.base.insert(index, value);
    }

    fn insert_to_check(&mut self, index: usize, value: usize) {
        let resized = cmp::max(self.check.len(), index);
        self.check.resize(resized, 0);
        self.check.insert(index, value);
    }

    fn find_s(&self, s: usize, children: &HashMap<char, TrieTree>) -> i32 {
        let mut position = s + 1;
        let offsets: Vec<_> = children.keys().map(|c| self.get_code(c).unwrap()).collect();
        while !offsets
            .iter()
            .map(|code| *self.check.get(position + code).unwrap_or(&0))
            .all(|n| n == 0)
        {
            position += 1;
        }
        position as i32
    }
}

fn as_usize(n: &i32) -> usize {
    assert!(*n >= 0, "n({}) should be greater than or equal to 0", n);
    *n as usize
}

fn concat_char_to_str(text: &String, c: char) -> String {
    let mut tmp = String::from(text);
    tmp.push(c);
    tmp
}

mod tests {
    use super::*;

    #[test]
    fn build_a_word() {
        let mut trie = TrieTree::new();
        trie.append(1, "あ");
        let da = DoubleArray::from_trie(&trie);
        // assert_eq!(extract("bc", &da).tokens.len(), 1);
    }
}
