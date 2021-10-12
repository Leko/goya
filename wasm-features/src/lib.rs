use goya::id::WordIdentifier;
use goya::word_features::WordFeaturesMap;
use rkyv::{archived_root, Deserialize, Infallible};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WORD_FEATURES: WordFeaturesMap = {
        let archived = unsafe {
            archived_root::<WordFeaturesMap>(include_bytes!("../__generated__/features.bin"))
        };
        archived.deserialize(&mut Infallible).unwrap()
    };
}

#[wasm_bindgen]
pub fn get_features(wids: &str) -> JsValue {
    let wids: Vec<WordIdentifier> = serde_json::from_str(wids).unwrap();
    let features: Vec<Vec<String>> = wids
        .iter()
        .map(|wid| {
            WORD_FEATURES
                .get(wid)
                .unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect()
        })
        .collect::<Vec<_>>();
    serde_wasm_bindgen::to_value(&features).unwrap()
}
