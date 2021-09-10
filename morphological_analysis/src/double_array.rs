use super::trie_tree::TrieTree;
use indexmap::IndexSet;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp;
use std::collections::HashMap;
use std::convert::TryInto;

const INDEX_ROOT: usize = 1;

#[derive(Debug)]
pub struct MatchResult {
    pub next_state: usize,
    pub can_stop: bool,
}

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

    pub fn from_trie(trie: &TrieTree) -> DoubleArray {
        let mut state_cache = HashMap::new();
        let mut da = DoubleArray::new();
        for (prefix, node) in trie.entires() {
            let c = prefix.chars().last().unwrap_or('\0');
            // FIXME: Is doing it here the best?
            // We might have calculate it before appending to base
            let char_code = da.insert_to_codes(c);
            // root node
            if prefix.is_empty() {
                for next_c in node.children.keys().sorted() {
                    let next_char_code = da.insert_to_codes(*next_c);
                    let t = da.base[INDEX_ROOT] + next_char_code as i32;
                    let t = as_usize(&t);
                    da.check.insert(t, INDEX_ROOT);
                    state_cache.insert(concat_char_to_str(&prefix, *next_c), t);
                }
                continue;
            }
            let s = *state_cache
                .get(&prefix)
                .unwrap_or_else(|| panic!("Unknown prefix: {:?} {:#?}", prefix, state_cache));
            if node.can_stop() {
                da.insert_to_base(s, node.id.unwrap() as i32 * -1);
                continue;
            }
            for next_c in node.children.keys().sorted() {
                da.insert_to_codes(*next_c);
            }
            da.insert_to_base(s, da.find_s(&node.children));
            for next_c in node.children.keys().sorted() {
                let child = node.children.get(next_c).unwrap();
                let t = da.base.get(s).unwrap() + char_code as i32;
                let t: usize = as_usize(&t);
                da.insert_to_check(t, s);
                let key = concat_char_to_str(&prefix, *next_c);
                state_cache.insert(key, t);
                if child.can_stop() {
                    da.insert_to_base(t, child.id.unwrap() as i32 * -1);
                }
            }
        }
        da.base.shrink_to_fit();
        da.check.shrink_to_fit();
        da
    }

    pub fn match_char(&self, s: usize, c: &char) -> Option<MatchResult> {
        if self.base.get(s).unwrap_or(&-1) < &0 {
            return None;
        }
        match self.next(s, c) {
            Some(t) => match self.check.get(t).unwrap_or(&0) == &s {
                true => Some(MatchResult {
                    next_state: t,
                    can_stop: self.match_char(t, &'\0').is_some(),
                }),
                _ => None,
            },
            None => None,
        }
    }

    fn next(&self, s: usize, c: &char) -> Option<usize> {
        match self.codes.get(c) {
            Some(code) => {
                let next = self.base[s] + *code as i32;
                if next < 0 {
                    return None;
                }
                Some((next).try_into().unwrap())
            }
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

    fn find_s(&self, children: &HashMap<char, TrieTree>) -> i32 {
        let mut position = INDEX_ROOT;
        let codes: Vec<_> = children
            .iter()
            .map(|(c, _)| self.codes.get_full(c).unwrap().0)
            .collect();
        while !codes
            .iter()
            .map(|offset| *self.check.get(position + offset).unwrap_or(&0))
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
