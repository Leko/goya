use super::path_util::PathUtil;
use bytesize::ByteSize;
use console::{style, Emoji};
use morphological_analysis::common_prefix_tree::CommonPrefixTree;
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::IPADic;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::time::Instant;

const LOOKING_GLASS: Emoji = Emoji("🔍", "");
const PAPER: Emoji = Emoji("📃", "");
const CLIP: Emoji = Emoji("🔗", "");
const SPARKLE: Emoji = Emoji("✨", "");
const TRUCK: Emoji = Emoji("🚚", "");

pub fn build(src_dir: &str, dist_dir: &str) -> Result<(), Box<dyn Error>> {
    PathUtil::from(dist_dir.to_string());
    let timer = Instant::now();
    eprintln!(
        "{} {} Loading dictionary...",
        style("[1/4]").bold().dim(),
        LOOKING_GLASS
    );
    let ipadic = IPADic::from_dir(src_dir)?;

    eprintln!(
        "{} {} Analyzing vocabulary...",
        style("[2/4]").bold().dim(),
        PAPER
    );
    let mut cpt = CommonPrefixTree::default();
    for (id, word) in ipadic.vocabulary.iter() {
        cpt.append(*id, &word.surface_form);
    }

    eprintln!(
        "{} {} Recompiling dictionary...",
        style("[3/4]").bold().dim(),
        CLIP
    );
    // let guard = pprof::ProfilerGuard::new(100).unwrap();
    let da = DoubleArray::from_cpt(&cpt);
    // if let Ok(report) = guard.report().build() {
    //     let file = File::create("flamegraph.svg").unwrap();
    //     report.flamegraph(file).unwrap();
    // };

    eprintln!(
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

    eprintln!("DoubleArray stats:");
    eprintln!("  elements: {}", da.base.len());
    eprintln!(
        "  bytes: {}",
        ByteSize(bincode::serialized_size(&da).unwrap())
    );
    eprintln!("Dictionary stats:");
    eprintln!(
        "  bytes: {}",
        ByteSize(bincode::serialized_size(&ipadic).unwrap())
    );

    let end = timer.elapsed();
    eprintln!(
        "{} Done in {}.{:03}s",
        SPARKLE,
        end.as_secs(),
        end.subsec_millis()
    );
    Ok(())
}
