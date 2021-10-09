use goya::dot;
use goya::double_array::DoubleArray;
use goya::id::WordIdentifier;
use goya::lattice::Lattice;
use goya::{dictionary::Dictionary, word_features::WordFeaturesMap};
use goya_ipadic::ipadic::IPADic;
use rkyv::{archived_root, Deserialize, Infallible};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DOUBLE_ARRAY: DoubleArray = {
        let archived =
            unsafe { archived_root::<DoubleArray>(include_bytes!("../__generated__/da.bin")) };
        archived.deserialize(&mut Infallible).unwrap()
    };
    static ref IPADIC: IPADic = {
        let archived =
            unsafe { archived_root::<IPADic>(include_bytes!("../__generated__/dict.bin")) };
        archived.deserialize(&mut Infallible).unwrap()
    };
    static ref WORD_FEATURES: WordFeaturesMap = {
        let archived = unsafe {
            archived_root::<WordFeaturesMap>(include_bytes!("../__generated__/features.bin"))
        };
        archived.deserialize(&mut Infallible).unwrap()
    };
}

#[derive(Serialize)]
pub struct WasmMorpheme {
    surface_form: String,
    left_context_id: usize,
    right_context_id: usize,
    cost: i16,
    features: Vec<String>,
}
impl WasmMorpheme {}

#[wasm_bindgen]
pub struct WasmLattice {
    lattice: Lattice,
}
#[wasm_bindgen]
impl WasmLattice {
    pub fn as_dot(&self) -> String {
        dot::render(&self.lattice, &*IPADIC).unwrap()
    }

    pub fn wakachi(&self) -> Vec<JsValue> {
        self.best_morphemes()
            .map(|morpheme| serde_wasm_bindgen::to_value(&morpheme.surface_form).unwrap())
            .collect()
    }

    pub fn find_best(&self) -> Vec<JsValue> {
        self.best_morphemes()
            .map(|morpheme| serde_wasm_bindgen::to_value(&morpheme).unwrap())
            .collect()
    }

    fn best_morphemes(&self) -> impl Iterator<Item = WasmMorpheme> + '_ {
        self.lattice
            .find_best()
            .map(|path| {
                path.into_iter().map(|wid| {
                    let morpheme = IPADIC.get(&wid).unwrap();
                    let features = WORD_FEATURES
                        .get(&wid)
                        .unwrap()
                        .iter()
                        .map(|s| s.to_string())
                        .collect();
                    let surface_form = match wid {
                        WordIdentifier::Known(_, s) => s,
                        WordIdentifier::Unknown(_, s) => s,
                    };
                    WasmMorpheme {
                        surface_form,
                        features,
                        left_context_id: morpheme.left_context_id,
                        right_context_id: morpheme.right_context_id,
                        cost: morpheme.cost,
                    }
                })
            })
            .unwrap()
    }
}

#[wasm_bindgen]
pub fn ready() {
    lazy_static::initialize(&DOUBLE_ARRAY);
    lazy_static::initialize(&IPADIC);
    lazy_static::initialize(&WORD_FEATURES);
}

#[wasm_bindgen]
pub fn parse(text: &str) -> WasmLattice {
    WasmLattice {
        lattice: Lattice::parse(text, &DOUBLE_ARRAY, &*IPADIC),
    }
}
