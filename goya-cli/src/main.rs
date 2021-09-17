mod repl;

use indicatif::{ProgressBar, ProgressStyle};
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::IPADic;
use morphological_analysis::trie_tree::TrieTree;
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

            let encoded = std::fs::read("./ipadic.bin").expect("Failed to load vocabulary");
            let ipadic: IPADic = bincode::deserialize(&encoded[..]).unwrap();

            repl::start(da, &ipadic)
        }
    }
}

fn build(path: &String) -> Result<(), Box<dyn Error>> {
    let ipadic = IPADic::load_dir(path)?;
    let mut trie = TrieTree::new();
    for (id, word) in &ipadic.vocabulary {
        trie.append(*id, &word.surface_form);
    }

    let pb = ProgressBar::new(ipadic.vocabulary.len() as u64);
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

    let encoded = bincode::serialize(&ipadic).unwrap();
    std::fs::write("./ipadic.bin", encoded).expect("Failed to write dictionary");

    Ok(())
}
