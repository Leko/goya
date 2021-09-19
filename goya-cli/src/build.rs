use super::path_util::PathUtil;
use console::{style, Emoji};
use indicatif::{ProgressBar, ProgressStyle};
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::IPADic;
use morphological_analysis::trie_tree::TrieTree;
use std::error::Error;
use std::fs;
use std::time::Instant;

const LOOKING_GLASS: Emoji = Emoji("🔍", "");
const PAPER: Emoji = Emoji("📃", "");
const CLIP: Emoji = Emoji("🔗", "");
const SPARKLE: Emoji = Emoji("✨", "");
const TRUCK: Emoji = Emoji("🚚", "");

pub fn build(src_dir: &String, dist_dir: &String) -> Result<(), Box<dyn Error>> {
    PathUtil::from(dist_dir.to_string());
    let timer = Instant::now();
    println!(
        "{} {} Loading dictionary...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    let ipadic = IPADic::load_dir(src_dir)?;

    println!(
        "{} {} Analyzing vocabulary...",
        style("[2/4]").bold().dim(),
        PAPER
    );
    let mut trie = TrieTree::new();
    for (id, word) in &ipadic.vocabulary {
        trie.append(*id, &word.surface_form);
    }

    println!(
        "{} {} Recompiling dictionary...",
        style("[3/4]").bold().dim(),
        CLIP
    );
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
        "{} {} Exporting dictionary...",
        style("[4/4]").bold().dim(),
        TRUCK
    );
    let util = PathUtil::from(dist_dir.to_string());
    util.mkdirp().expect("Failed to create directory");
    fs::write(util.da_path(), bincode::serialize(&da).unwrap())
        .expect("Failed to write dictionary");
    fs::write(util.dict_path(), bincode::serialize(&ipadic).unwrap())
        .expect("Failed to write dictionary");

    let end = timer.elapsed();
    println!(
        "{} Done in {}.{:03}s",
        SPARKLE,
        end.as_secs(),
        end.subsec_millis()
    );
    Ok(())
}
