use goya::id::WordIdentifier;
use goya::word_features::WordFeaturesMap;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref WORD_FEATURES: WordFeaturesMap =
        rmp_serde::from_slice(include_bytes!("../__generated__/features.bin")).unwrap();
}

#[wasm_bindgen]
pub fn get_features(wids: &JsValue) -> JsValue {
    let wids: Vec<WordIdentifier> = wids.into_serde().unwrap();
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

#[wasm_bindgen]
pub fn ready() {
    lazy_static::initialize(&WORD_FEATURES);
}
