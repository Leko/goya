use super::id::WordIdentifier;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::str::from_utf8_unchecked;

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct WordFeaturesMap {
    #[serde(with = "serde_bytes")]
    index: Vec<u8>,
    offsets: Vec<usize>,
    known: Vec<WordFeatures>,   // index = morpheme ID
    unknown: Vec<WordFeatures>, // index = morpheme ID
}
impl WordFeaturesMap {
    pub fn new(known: Vec<Vec<String>>, unknown: Vec<Vec<String>>) -> WordFeaturesMap {
        let mut tmp_index: IndexSet<String> = IndexSet::new();
        for features in known.iter().chain(unknown.iter()) {
            for f in features.iter() {
                tmp_index.insert(f.to_string());
            }
        }
        let mut index = vec![];
        let mut offsets: Vec<usize> = vec![0; tmp_index.len()];
        offsets[0] = tmp_index.get_index(0).unwrap().as_bytes().len();
        for (idx, str) in tmp_index.iter().enumerate() {
            index.append(&mut str.to_string().into_bytes());
            if idx > 0 {
                offsets[idx] = offsets[idx - 1] + str.as_bytes().len();
            }
        }

        WordFeaturesMap {
            known: known
                .into_iter()
                .map(|f| {
                    WordFeatures::new(f.iter().map(|s| tmp_index.get_full(s).unwrap().0).collect())
                })
                .collect(),
            unknown: unknown
                .into_iter()
                .map(|f| {
                    WordFeatures::new(f.iter().map(|s| tmp_index.get_full(s).unwrap().0).collect())
                })
                .collect(),
            index,
            offsets,
        }
    }

    pub fn get(&self, wid: &WordIdentifier) -> Option<Vec<&str>> {
        match wid {
            WordIdentifier::Known(wid, _) => self.get_known(wid),
            WordIdentifier::Unknown(wid, _) => self.get_unknown(wid),
        }
    }

    pub fn get_known(&self, wid: &usize) -> Option<Vec<&str>> {
        self.known.get(*wid).map(|f| self.get_string(f))
    }

    pub fn get_unknown(&self, wid: &usize) -> Option<Vec<&str>> {
        self.unknown.get(*wid).map(|f| self.get_string(f))
    }

    fn get_string(&self, f: &WordFeatures) -> Vec<&str> {
        f.0.iter()
            .map(|idx| {
                let idx = *idx;
                let end = self.offsets[idx];
                if idx == 0 {
                    unsafe { from_utf8_unchecked(&self.index[0..end]) }
                } else {
                    unsafe { from_utf8_unchecked(&self.index[(self.offsets[idx - 1])..end]) }
                }
            })
            .collect()
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
