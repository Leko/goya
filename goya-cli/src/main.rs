mod repl;

use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic;
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(dir) => match build(dir) {
            Ok(words) => {
                println!("{:?}", words);
            }
            Err(err) => {
                println!("{:?}", err);
            }
        },
        None => repl::start(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TrieTree {
    stop: bool,
    children: HashMap<char, Box<TrieTree>>,
}

impl TrieTree {
    pub fn new() -> TrieTree {
        TrieTree {
            stop: false,
            children: HashMap::new(),
        }
    }

    fn append_chars(&mut self, text: &str, cursor: usize) {
        let c = text.chars().nth(cursor).unwrap();
        if let None = self.children.get(&c) {
            self.children.insert(c, Box::from(TrieTree::new()));
        }
        let child = self.children.get_mut(&c).unwrap();
        if cursor + 1 == text.chars().count() {
            child.stop = true;
            return;
        }
        child.append_chars(text, cursor + 1);
    }

    pub fn append(&mut self, word: &str) {
        self.append_chars(word, 0);
    }
}

fn build(path: &String) -> Result<DoubleArray, Box<dyn Error>> {
    let base: Vec<i32> = vec![0, 1];
    let check: Vec<usize> = vec![0, 0];

    let words = ipadic::load_dir(path)?;
    println!("{}", words.len());
    // println!("{:?}", words.get(1));

    let mut trie = TrieTree::new();
    for w in words {
        trie.append(&w.surface_form);
    }
    println!("{:#?}", trie);

    // registered words: "a" and "bc"
    let mut codes: HashMap<char, usize> = HashMap::new();
    codes.insert('\0', 0);
    codes.insert('a', 1);
    codes.insert('b', 2);
    codes.insert('c', 3);
    let base: Vec<i32> = vec![0, 3, 0, -1, 3, 3, 7, -1];
    let check: Vec<usize> = vec![0, 0, 0, 4, 1, 1, 5, 6];

    return Ok(DoubleArray::new(base, check, codes));
}
