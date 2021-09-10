mod repl;

use indexmap::set::IndexSet;
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic;
use morphological_analysis::trie_tree::TrieTree;
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(dir) => match build(dir) {
            Ok(dict) => {
                let encoded = bincode::serialize(&dict).unwrap();
                std::fs::write("./cache.bin", encoded).expect("Failed to write dictionary");
            }
            Err(err) => {
                println!("{:?}", err);
            }
        },
        None => {
            let encoded = std::fs::read("./cache.bin").expect("Failed to write dictionary");
            let dict = bincode::deserialize(&encoded[..]).unwrap();
            repl::start(&dict)
        }
    }
}

fn build(path: &String) -> Result<DoubleArray, Box<dyn Error>> {
    let words = ipadic::load_dir(path)?;
    let mut dict = HashMap::new();
    let mut trie = TrieTree::new();
    for (idx, word) in words.iter().enumerate() {
        let id = idx + 1;
        dict.insert(id, word);
        trie.append(id, &word.surface_form);
    }
    Ok(DoubleArray::from_trie(&trie))
}

fn demodict() -> DoubleArray {
    // registered words: "a" and "bc"
    let mut codes: IndexSet<char> = IndexSet::new();
    codes.insert('\0');
    codes.insert('a');
    codes.insert('b');
    codes.insert('c');
    let base: Vec<i32> = vec![0, 3, 0, -1, 3, 3, 7, -1];
    let check: Vec<usize> = vec![0, 0, 0, 4, 1, 1, 5, 6];
    return DoubleArray::from(base, check, codes);
}
