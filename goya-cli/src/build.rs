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

    let bytes = rmp_serde::to_vec(&loaded.word_set).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use goya::word_features::WordFeaturesMap;
    use goya_ipadic::ipadic_loader::LoadResult;
    use rkyv::{archived_root, Infallible};
    use serde::{Deserialize, Serialize};
    use std::io::Write;
    use std::{env, path::PathBuf};
    use test::Bencher;

    lazy_static! {
        static ref LOADED: LoadResult = {
            let mut dict_path = PathBuf::from(env::current_dir().unwrap());
            dict_path.pop();
            dict_path.push("mecab-ipadic-2.7.0-20070801");
            let loader = IPADicLoader {};
            let loaded = loader.load(dict_path.as_path().to_str().unwrap()).unwrap();
            features_binary_sizes(&loaded);
            loaded
        };
    }

    fn features_binary_sizes(loaded: &LoadResult) {
        let stderr = &mut std::io::stderr();
        let mut serializer = AllocSerializer::<256>::default();
        serializer.serialize_value(&loaded.word_set).unwrap();
        writeln!(
            stderr,
            "features_rkyv: {}",
            ByteSize(serializer.into_serializer().into_inner().len() as u64)
        )
        .unwrap();

        writeln!(
            stderr,
            "features_json: {}",
            ByteSize(serde_json::to_vec(&loaded.word_set).unwrap().len() as u64)
        )
        .unwrap();

        writeln!(
            stderr,
            "features_bincode: {}",
            ByteSize(bincode::serialize(&loaded.word_set).unwrap().len() as u64)
        )
        .unwrap();

        writeln!(
            stderr,
            "features_cbor: {}",
            ByteSize(serde_cbor::to_vec(&loaded.word_set).unwrap().len() as u64)
        )
        .unwrap();

        writeln!(
            stderr,
            "features_rmp: {}",
            ByteSize(rmp_serde::to_vec(&loaded.word_set).unwrap().len() as u64)
        )
        .unwrap();

        writeln!(
            stderr,
            "features_pickle: {}",
            ByteSize(
                serde_pickle::to_vec(&loaded.word_set, Default::default())
                    .unwrap()
                    .len() as u64
            )
        )
        .unwrap();

        writeln!(
            stderr,
            "features_postcard: {}",
            ByteSize(postcard::to_allocvec(&loaded.word_set).unwrap().len() as u64)
        )
        .unwrap();

        let mut s = flexbuffers::FlexbufferSerializer::new();
        loaded.word_set.serialize(&mut s).unwrap();
        writeln!(
            stderr,
            "features_flexbuffers: {}",
            ByteSize(s.view().len() as u64)
        )
        .unwrap();
    }

    #[bench]
    fn features_rkyv_deserialize(b: &mut Bencher) {
        use rkyv::Deserialize;

        lazy_static::initialize(&LOADED);
        let mut serializer = AllocSerializer::<256>::default();
        serializer.serialize_value(&LOADED.word_set).unwrap();
        let bytes = serializer.into_serializer().into_inner();

        b.iter(|| {
            let archived = unsafe { archived_root::<WordFeaturesMap>(&bytes[..]) };
            let _x: WordFeaturesMap = archived.deserialize(&mut Infallible).unwrap();
        });
    }

    #[bench]
    fn features_json_deserialize(b: &mut Bencher) {
        let bytes = serde_json::to_vec(&LOADED.word_set).unwrap();
        b.iter(|| {
            let _x: WordFeaturesMap = serde_json::from_slice(&bytes).unwrap();
        });
    }

    #[bench]
    fn features_bincode_deserialize(b: &mut Bencher) {
        let bytes = bincode::serialize(&LOADED.word_set).unwrap();
        b.iter(|| {
            let _x: WordFeaturesMap = bincode::deserialize(&bytes).unwrap();
        });
    }

    #[bench]
    fn features_cbor_deserialize(b: &mut Bencher) {
        let bytes = serde_cbor::to_vec(&LOADED.word_set).unwrap();
        b.iter(|| {
            let _x: WordFeaturesMap = serde_cbor::from_slice(&bytes).unwrap();
        });
    }

    #[bench]
    fn features_rmp_deserialize(b: &mut Bencher) {
        let bytes = rmp_serde::to_vec(&LOADED.word_set).unwrap();
        b.iter(|| {
            let _x: WordFeaturesMap = rmp_serde::from_slice(&bytes).unwrap();
        });
    }

    #[bench]
    fn features_pickle_deserialize(b: &mut Bencher) {
        let bytes = serde_pickle::to_vec(&LOADED.word_set, Default::default()).unwrap();
        b.iter(|| {
            let _x: WordFeaturesMap = serde_pickle::from_slice(&bytes, Default::default()).unwrap();
        });
    }

    #[bench]
    fn features_postcard_deserialize(b: &mut Bencher) {
        let bytes = postcard::to_allocvec(&LOADED.word_set).unwrap();
        b.iter(|| {
            let _x: WordFeaturesMap = postcard::from_bytes(&bytes).unwrap();
        });
    }

    #[bench]
    fn features_flexbuffers_deserialize(b: &mut Bencher) {
        let mut s = flexbuffers::FlexbufferSerializer::new();
        LOADED.word_set.serialize(&mut s).unwrap();
        b.iter(|| {
            let r = flexbuffers::Reader::get_root(s.view()).unwrap();
            let _x = WordFeaturesMap::deserialize(r).unwrap();
        });
    }
}
