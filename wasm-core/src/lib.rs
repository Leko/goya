use goya::dot;
use goya::double_array::DoubleArray;
use goya::id::WordIdentifier;
use goya::lattice::Lattice;
use goya::{dictionary::Dictionary, word_features::WordFeaturesMap};
use goya_ipadic::ipadic::IPADic;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GoyaContext {
    da: DoubleArray,
    dict: IPADic,
    features: WordFeaturesMap,
}
#[wasm_bindgen]
impl GoyaContext {
    #[wasm_bindgen(constructor)]
    pub fn new(da: &str, dict: &str, features: &str) -> GoyaContext {
        let da: DoubleArray = serde_json::from_str(da).unwrap();
        let dict: IPADic = serde_json::from_str(dict).unwrap();
        let features: WordFeaturesMap = if features.is_empty() {
            WordFeaturesMap::new(vec![], vec![])
        } else {
            serde_json::from_str(features).unwrap()
        };
        GoyaContext { da, dict, features }
    }
}

#[derive(Serialize)]
pub struct WasmMorpheme {
    wid: WordIdentifier,
    is_known: bool,
    surface_form: String,
    left_context_id: usize,
    right_context_id: usize,
    cost: i16,
}
impl WasmMorpheme {}

#[wasm_bindgen]
pub struct WasmLattice {
    lattice: Lattice,
}
#[wasm_bindgen]
impl WasmLattice {
    pub fn as_dot(&self, context: &GoyaContext) -> String {
        dot::render(&self.lattice, &context.dict).unwrap()
    }

    pub fn wakachi(&self, context: &GoyaContext) -> Vec<JsValue> {
        self.best_morphemes(context)
            .map(|morpheme| serde_wasm_bindgen::to_value(&morpheme.surface_form).unwrap())
            .collect()
    }

    pub fn find_best(&self, context: &GoyaContext) -> Vec<JsValue> {
        self.best_morphemes(context)
            .map(|morpheme| serde_wasm_bindgen::to_value(&morpheme).unwrap())
            .collect()
    }

    fn best_morphemes<'a>(
        &self,
        context: &'a GoyaContext,
    ) -> impl Iterator<Item = WasmMorpheme> + 'a {
        self.lattice
            .find_best()
            .map(move |path| {
                path.into_iter().map(move |wid| {
                    let morpheme = context.dict.get(&wid).unwrap();
                    let (surface_form, is_known) = match &wid {
                        WordIdentifier::Known(_, s) => (s.to_string(), true),
                        WordIdentifier::Unknown(_, s) => (s.to_string(), false),
                    };
                    WasmMorpheme {
                        wid,
                        is_known,
                        surface_form,
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
pub fn parse(text: &str, context: &GoyaContext) -> WasmLattice {
    WasmLattice {
        lattice: Lattice::parse(text, &context.da, &context.dict),
    }
}

#[wasm_bindgen]
pub fn get_features(wids: &str, context: &GoyaContext) -> JsValue {
    let wids: Vec<WordIdentifier> = serde_json::from_str(wids).unwrap();
    let features: Vec<Option<Vec<String>>> = wids
        .iter()
        .map(move |wid| {
            context
                .features
                .get(wid)
                .map(|f| f.iter().map(|s| s.to_string()).collect())
        })
        .collect::<Vec<_>>();
    serde_wasm_bindgen::to_value(&features).unwrap()
}
