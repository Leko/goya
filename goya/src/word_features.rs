use super::id::WordIdentifier;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordFeaturesMap {
    index: IndexSet<String>,
    known: Vec<WordFeatures>,   // index = morpheme ID
    unknown: Vec<WordFeatures>, // index = morpheme ID
}
impl WordFeaturesMap {
    pub fn new(known: Vec<Vec<String>>, unknown: Vec<Vec<String>>) -> WordFeaturesMap {
        let mut index: IndexSet<String> = IndexSet::new();
        for features in known.iter().chain(unknown.iter()) {
            for f in features.iter() {
                index.insert(f.to_string());
            }
        }

        WordFeaturesMap {
            known: known
                .into_iter()
                .map(|f| {
                    WordFeatures::new(f.iter().map(|s| index.get_full(s).unwrap().0).collect())
                })
                .collect(),
            unknown: unknown
                .into_iter()
                .map(|f| {
                    WordFeatures::new(f.iter().map(|s| index.get_full(s).unwrap().0).collect())
                })
                .collect(),
            index,
        }
    }

    pub fn get(&self, wid: &WordIdentifier) -> Option<Vec<&String>> {
        match wid {
            WordIdentifier::Known(wid, _) => self.get_known(wid),
            WordIdentifier::Unknown(wid, _) => self.get_unknown(wid),
        }
    }

    pub fn get_known(&self, wid: &usize) -> Option<Vec<&String>> {
        self.known.get(*wid).map(|f| {
            f.0.iter()
                .map(|idx| self.index.get_index(*idx).unwrap())
                .collect()
        })
    }

    pub fn get_unknown(&self, wid: &usize) -> Option<Vec<&String>> {
        self.unknown.get(*wid).map(|f| {
            f.0.iter()
                .map(|idx| self.index.get_index(*idx).unwrap())
                .collect()
        })
    }
}

/// > 5カラム目以降は, ユーザ定義の CSV フィールドです. 基本的に どんな内容でも CSV の許す限り追加することができます.
/// > https://taku910.github.io/mecab/dic-detail.html
#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordFeatures(Vec<usize>);
impl WordFeatures {
    pub fn new(features: Vec<usize>) -> WordFeatures {
        WordFeatures(features)
    }
}
