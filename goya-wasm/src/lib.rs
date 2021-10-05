use morphological_analysis::dot;
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::{IPADic, WordIdentifier};
use morphological_analysis::lattice::Lattice;
use morphological_analysis::morpheme::Morpheme;
use rkyv::{archived_root, Deserialize, Infallible};
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
}

#[wasm_bindgen]
pub struct WasmLattice {
    lattice: Lattice,
}

#[wasm_bindgen]
impl WasmLattice {
    pub fn as_dot(&self) -> String {
        dot::render(&self.lattice, &IPADIC).unwrap()
    }

    pub fn find_best(&self) -> String {
        let mut best = vec![];
        if let Some(path) = self.lattice.find_best() {
            for wid in path.into_iter() {
                let word = IPADIC.get_word(&wid).unwrap();
                if let WordIdentifier::Unknown(_, surface_form) = wid {
                    let actual = Morpheme {
                        surface_form,
                        ..word.clone()
                    };
                    best.push(actual);
                } else {
                    best.push(word.clone());
                }
            }
        }
        serde_json::to_string(&best).unwrap()
    }
}

#[wasm_bindgen]
pub fn ready() {
    lazy_static::initialize(&DOUBLE_ARRAY);
    lazy_static::initialize(&IPADIC);
}

#[wasm_bindgen]
pub fn parse(text: &str) -> WasmLattice {
    WasmLattice {
        lattice: Lattice::parse(text, &DOUBLE_ARRAY, &IPADIC),
    }
}
