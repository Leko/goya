use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordFeaturesMap {
    known: HashMap<usize, WordFeatures>,
    unknown: HashMap<usize, WordFeatures>,
}
impl WordFeaturesMap {
    pub fn new(
        known: HashMap<usize, WordFeatures>,
        unknown: HashMap<usize, WordFeatures>,
    ) -> WordFeaturesMap {
        WordFeaturesMap { known, unknown }
    }

    pub fn get_known(&self, wid: &usize) -> Option<&WordFeatures> {
        self.known.get(wid)
    }

    pub fn get_unknown(&self, wid: &usize) -> Option<&WordFeatures> {
        self.unknown.get(wid)
    }

    pub fn known_words(&self) -> impl Iterator<Item = (&usize, &WordFeatures)> {
        self.known.iter()
    }
}

/// > 5カラム目以降は, ユーザ定義の CSV フィールドです. 基本的に どんな内容でも CSV の許す限り追加することができます.
/// > https://taku910.github.io/mecab/dic-detail.html
#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordFeatures(Vec<String>);
impl WordFeatures {
    pub fn new(features: Vec<String>) -> WordFeatures {
        WordFeatures(features)
    }

    pub fn join(&self, sep: &str) -> String {
        self.0.join(sep)
    }
}
