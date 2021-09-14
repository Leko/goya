mod repl;

use indicatif::{ProgressBar, ProgressStyle};
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic;
use morphological_analysis::trie_tree::TrieTree;
use morphological_analysis::vocabulary::Word;
use std::collections::HashMap;
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(dir) => match build(dir) {
            Ok(_) => {
                println!("DONE");
            }
            Err(err) => {
                println!("{:?}", err);
            }
        },
        None => {
            let encoded = std::fs::read("./da.bin").expect("Failed to load dictionary");
            let da = bincode::deserialize(&encoded[..]).unwrap();

            let encoded = std::fs::read("./vocab.bin").expect("Failed to load vocabulary");
            let vocab: HashMap<usize, Word> = bincode::deserialize(&encoded[..]).unwrap();

            repl::start(da, vocab)
        }
    }
}

fn build(path: &String) -> Result<(), Box<dyn Error>> {
    let words = ipadic::load_dir(path)?;
    let mut dict = HashMap::new();
    let mut trie = TrieTree::new();
    for (idx, word) in words.iter().enumerate() {
        let id = idx + 1;
        dict.insert(id, word);
        trie.append(id, &word.surface_form);
    }

    let pb = ProgressBar::new(words.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar}] {pos:>7}/{len:7} ({eta})")
            .progress_chars("#>-"),
    );
    let da = DoubleArray::from_trie(&trie, |(completed, total)| {
        pb.set_length(total as u64);
        pb.set_position(completed as u64);
    });

    let encoded = bincode::serialize(&da).unwrap();
    std::fs::write("./da.bin", encoded).expect("Failed to write dictionary");

    let encoded = bincode::serialize(&dict).unwrap();
    std::fs::write("./vocab.bin", encoded).expect("Failed to write dictionary");

    Ok(())
}
