mod repl;

use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::IPADic;
use morphological_analysis::trie_tree::TrieTree;
use std::env;
use std::error::Error;
use std::time::Instant;

const LOOKING_GLASS: Emoji = Emoji("🔍", "");
const PAPER: Emoji = Emoji("📃", "");
const CLIP: Emoji = Emoji("🔗", "");
const SPARKLE: Emoji = Emoji("✨", ":-)");
const TRUCK: Emoji = Emoji("🚚", "");

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(dir) => {
            let timer = Instant::now();
            match build(dir) {
                Ok(_) => {
                    let end = timer.elapsed();
                    println!(
                        "{} Done in {}.{:03}s",
                        SPARKLE,
                        end.as_secs(),
                        end.subsec_millis()
                    );
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
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
    println!(
        "{} {} Loading dictionary...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    let ipadic = IPADic::load_dir(path)?;

    println!(
        "{} {} Analyzing vocabulary...",
        style("[2/4]").bold().dim(),
        PAPER
    );
    let mut trie = TrieTree::new();
    for (id, word) in &ipadic.vocabulary {
        trie.append(*id, &word.surface_form);
    }

    println!("{} {} Building cache...", style("[3/4]").bold().dim(), CLIP);
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
    pb.finish_and_clear();

    println!(
        "{} {} Exporting cache...",
        style("[4/4]").bold().dim(),
        TRUCK
    );
    let encoded = bincode::serialize(&da).unwrap();
    std::fs::write("./da.bin", encoded).expect("Failed to write dictionary");

    let encoded = bincode::serialize(&ipadic).unwrap();
    std::fs::write("./ipadic.bin", encoded).expect("Failed to write dictionary");

    Ok(())
}
