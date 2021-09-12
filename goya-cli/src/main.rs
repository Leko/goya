mod repl;

use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic;
use morphological_analysis::trie_tree::TrieTree;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::time::Instant;

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
            let encoded = std::fs::read("./cache.bin").expect("Failed to load dictionary");
            let dict = bincode::deserialize(&encoded[..]).unwrap();

            // let mut trie = TrieTree::new();
            // trie.append(1, "あ");
            // trie.append(2, "a");
            // let dict = DoubleArray::from_trie(&trie);

            // println!("{:#?}", trie);
            println!("{:#?}", dict);
            repl::start(&dict)
        }
    }
}

fn build(path: &String) -> Result<DoubleArray, Box<dyn Error>> {
    let mut stats = vec![];
    for _ in 1..10 {
        let timer = Instant::now();
        ipadic::load_dir(path)?;
        let spent = timer.elapsed();
        stats.push(spent.as_millis());
    }
    println!(
        "sum={}ms, avg={}ms",
        stats.iter().sum::<u128>() as f32,
        stats.iter().sum::<u128>() as f32 / stats.iter().count() as f32
    );
    panic!("debug");

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
