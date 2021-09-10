use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
pub struct TrieTree {
    pub id: Option<usize>,
    pub children: HashMap<char, TrieTree>,
}

impl TrieTree {
    pub fn new() -> TrieTree {
        TrieTree {
            id: None,
            children: HashMap::new(),
        }
    }

    pub fn can_stop(&self) -> bool {
        self.id.is_some()
    }

    pub fn size(&self) -> usize {
        self.entires().count()
    }

    pub fn append(&mut self, id: usize, word: &str) {
        let mut token = String::from(word);
        token.push('\0');
        self.append_chars(id, &token, 0);
    }

    pub fn entires(&self) -> TrieTreeVisitor {
        let mut open = VecDeque::new();
        open.push_back((String::new(), self));
        TrieTreeVisitor { open }
    }

    fn append_chars(&mut self, id: usize, text: &str, cursor: usize) {
        let c = text.chars().nth(cursor).unwrap();
        if let None = self.children.get(&c) {
            self.children.insert(c, TrieTree::new());
        }
        let child = self.children.get_mut(&c).unwrap();
        if cursor + 1 == text.chars().count() {
            child.id = Some(id);
            return;
        }
        child.append_chars(id, text, cursor + 1);
    }
}

pub struct TrieTreeVisitor<'a> {
    open: VecDeque<(String, &'a TrieTree)>,
}
impl<'a> Iterator for TrieTreeVisitor<'a> {
    type Item = (String, &'a TrieTree);

    fn next(&mut self) -> Option<Self::Item> {
        match self.open.pop_front() {
            Some((prefix, subtree)) => {
                for c in subtree.children.keys().sorted() {
                    let node = subtree.children.get(c).unwrap();
                    let mut substr = String::from(&prefix);
                    substr.push(*c);
                    self.open.push_back((substr, &node));
                }
                Some((prefix, &subtree))
            }
            None => None,
        }
    }
}
