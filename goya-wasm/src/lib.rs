use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::{IPADic, WordIdentifier};
use morphological_analysis::lattice::Lattice;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref DOUBLE_ARRAY: DoubleArray =
        bincode::deserialize(include_bytes!("../__generated__/da.bin")).unwrap();
    static ref IPADIC: IPADic =
        bincode::deserialize(include_bytes!("../__generated__/dict.bin")).unwrap();
}

#[wasm_bindgen]
pub struct WasmLattice {
    lattice: Lattice,
    ipadic: IPADic,
}

#[wasm_bindgen]
impl WasmLattice {
    pub fn as_dot(&self) -> String {
        self.lattice.as_dot(&self.ipadic)
    }

    pub fn find_best(&self) -> String {
        let mut best = vec![];
        match self.lattice.find_best() {
            Some(path) => {
                for wid in path.into_iter() {
                    let word = self.ipadic.get_word(&wid).unwrap();
                    if let WordIdentifier::Unknown(_) = wid {
                        // TODO: Display actual matched unknown text
                        best.push(word);
                    } else {
                        best.push(word);
                    }
                }
            }
            _ => {}
        }
        serde_json::to_string(&best).unwrap()
    }
}

#[wasm_bindgen]
pub fn parse(text: &str) -> WasmLattice {
    let lattice = Lattice::parse(text, &DOUBLE_ARRAY, &IPADIC);
    WasmLattice {
        ipadic: IPADIC.shrink(lattice.word_identifiers()),
        lattice,
    }
}
