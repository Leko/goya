use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordSet {
    known: HashMap<usize, WordSurface>,
    unknown: HashMap<usize, WordSurface>,
}
impl WordSet {
    pub fn new(
        known: HashMap<usize, WordSurface>,
        unknown: HashMap<usize, WordSurface>,
    ) -> WordSet {
        WordSet { known, unknown }
    }

    pub fn get_known(&self, wid: &usize) -> Option<&WordSurface> {
        self.known.get(wid)
    }

    pub fn get_unknown(&self, wid: &usize) -> Option<&WordSurface> {
        self.unknown.get(wid)
    }

    pub fn known_words(&self) -> impl Iterator<Item = (&usize, &WordSurface)> {
        self.known.iter()
    }
}

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordSurface {
    /// 表層形
    /// https://taku910.github.io/mecab/dic-detail.html
    pub surface_form: String,
    /// > 5カラム目以降は, ユーザ定義の CSV フィールドです. 基本的に どんな内容でも CSV の許す限り追加することができます.
    /// > https://taku910.github.io/mecab/dic-detail.html
    pub features: Vec<String>,
}
impl WordSurface {
    pub fn new(surface_form: impl Into<String>, features: Vec<String>) -> WordSurface {
        WordSurface {
            surface_form: surface_form.into(),
            features,
        }
    }
}
