use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub struct CommonPrefixTree {
    pub id: Option<usize>,
    pub children: BTreeMap<char, CommonPrefixTree>,
}

impl CommonPrefixTree {
    pub fn new() -> CommonPrefixTree {
        CommonPrefixTree {
            id: None,
            children: BTreeMap::new(),
        }
    }

    pub fn can_stop(&self) -> bool {
        self.id.is_some()
    }

    pub fn size(&self) -> usize {
        self.entires_dfs().len()
    }

    pub fn min_char(&self) -> Option<&char> {
        self.children.keys().min()
    }

    pub fn append(&mut self, id: usize, word: &str) {
        let mut token = String::from(word);
        token.push('\0');
        self.append_chars(id, &token, 0);
    }

    pub fn entires_dfs(&self) -> VecDeque<(String, &CommonPrefixTree)> {
        self.dfs_collect(&String::new())
    }

    fn dfs_collect(&self, prefix: &String) -> VecDeque<(String, &CommonPrefixTree)> {
        let mut open = VecDeque::new();
        open.push_back((prefix.to_string(), self));
        for c in self.children.keys().sorted() {
            let child = self.children.get(c).unwrap();
            let mut substr = String::from(prefix);
            substr.push(*c);
            open.append(&mut child.dfs_collect(&substr));
        }
        open
    }

    fn append_chars(&mut self, id: usize, text: &str, cursor: usize) {
        let c = text.chars().nth(cursor).unwrap();
        if let None = self.children.get(&c) {
            self.children.insert(c, CommonPrefixTree::new());
        }
        let child = self.children.get_mut(&c).unwrap();
        if cursor + 1 == text.chars().count() {
            child.id = Some(id);
            return;
        }
        child.append_chars(id, text, cursor + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::CommonPrefixTree;

    #[test]
    fn builds_a_word_that_has_1_char() {
        let mut trie = CommonPrefixTree::new();
        trie.append(1, "あい");
        trie.append(2, "いう");
        assert_eq!(
            trie.entires_dfs()
                .iter()
                .map(|(p, _)| p)
                .collect::<Vec<_>>(),
            vec!["", "あ", "あい", "あい\0", "い", "いう", "いう\0"]
        );
    }
}
