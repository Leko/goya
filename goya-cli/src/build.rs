use super::path_util::PathUtil;
use bytesize::ByteSize;
use console::{style, Emoji};
use goya::common_prefix_tree::CommonPrefixTree;
use goya::double_array::DoubleArray;
use goya_ipadic::ipadic::IPADic;
use goya_ipadic::ipadic_loader::IPADicLoader;
use rkyv::ser::{serializers::AllocSerializer, Serializer};
use std::error::Error;
use std::fs;
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
    let loader = IPADicLoader {};
    let mut loaded = loader.load(src_dir)?;

    eprintln!(
        "{} {} Analyzing vocabulary...",
        style("[2/4]").bold().dim(),
        PAPER
    );
    let mut cpt = CommonPrefixTree::default();
    for (id, surface) in loaded.surfaces.iter() {
        cpt.append(*id, surface);
    }

    eprintln!(
        "{} {} Recompiling dictionary...",
        style("[3/4]").bold().dim(),
        CLIP
    );
    let da = DoubleArray::from_cpt(&cpt);

    // DoubleArray only has one ID per surface form.
    let used_wids = da.wids().collect();
    loaded.ipadic.shrink_to_wids(&used_wids);

    eprintln!(
        "{} {} Exporting dictionary...",
        style("[4/4]").bold().dim(),
        TRUCK
    );
    let util = PathUtil::from(dist_dir.to_string());
    util.mkdirp().expect("Failed to create directory");

    let mut serializer = AllocSerializer::<256>::default();
    serializer.serialize_value(&da).unwrap();
    let bytes = serializer.into_serializer().into_inner();
    fs::write(util.da_path(), &bytes).expect("Failed to write dictionary");
    eprintln!("DoubleArray stats:");
    eprintln!("  elements: {}", da.base.len());
    eprintln!("  bytes: {}", ByteSize(bytes.len() as u64));

    let mut serializer = AllocSerializer::<256>::default();
    serializer
        .serialize_value::<IPADic>(&loaded.ipadic)
        .unwrap();
    let bytes = serializer.into_serializer().into_inner();
    fs::write(util.dict_path(), &bytes).expect("Failed to write dictionary");
    eprintln!("Dictionary stats:");
    eprintln!("  bytes: {}", ByteSize(bytes.len() as u64));

    let mut serializer = AllocSerializer::<256>::default();
    serializer.serialize_value(&loaded.word_set).unwrap();
    let bytes = serializer.into_serializer().into_inner();
    fs::write(util.features_path(), &bytes).expect("Failed to write word features");
    eprintln!("Word features stats:");
    eprintln!("  bytes: {}", ByteSize(bytes.len() as u64));

    let end = timer.elapsed();
    eprintln!(
        "{} Done in {}.{:03}s",
        SPARKLE,
        end.as_secs(),
        end.subsec_millis()
    );
    Ok(())
}
