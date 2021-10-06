use super::char_class::CharClassifier;
use super::morpheme::Morpheme;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::vec::Vec;

// TODO: Make it newtype idiom
type MorphemeId = usize;

#[derive(Debug, Clone)]
pub enum WordIdentifier {
    Known(MorphemeId, String),   // ID, surface_form
    Unknown(MorphemeId, String), // ID, surface_form
}
impl WordIdentifier {
    pub fn get_surface(&self) -> &str {
        match self {
            Self::Known(_, surface) => surface,
            Self::Unknown(_, surface) => surface,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct IPADic {
    pub vocabulary: HashMap<usize, Morpheme>,
    homonyms: HashMap<MorphemeId, Vec<usize>>,
    // FIXME: Remove
    pub classes: CharClassifier,
    matrix: Vec<Vec<i16>>,
    /// 1つのカテゴリに複数の素性を定義してもかまいません. 学習後, 適切なコスト値が 自動的に与えられます.
    /// https://taku910.github.io/mecab/learn.html#config
    unknown_classes: HashMap<String, Vec<usize>>,
    unknown_vocabulary: HashMap<usize, Morpheme>,
}
impl IPADic {
    pub fn from(
        vocabulary: HashMap<usize, Morpheme>,
        homonyms: HashMap<MorphemeId, Vec<usize>>,
        classes: CharClassifier,
        matrix: Vec<Vec<i16>>,
        unknown_classes: HashMap<String, Vec<usize>>,
        unknown_vocabulary: HashMap<usize, Morpheme>,
    ) -> IPADic {
        IPADic {
            vocabulary,
            homonyms,
            classes,
            matrix,
            unknown_classes,
            unknown_vocabulary,
        }
    }

    pub fn get_word(&self, wid: &WordIdentifier) -> Option<&Morpheme> {
        match wid {
            WordIdentifier::Known(wid, _) => self.get_known_word(wid),
            WordIdentifier::Unknown(wid, _) => self.get_unknown_word(wid),
        }
    }

    pub fn get_known_word(&self, wid: &usize) -> Option<&Morpheme> {
        self.vocabulary.get(wid)
    }

    pub fn get_unknown_word(&self, wid: &usize) -> Option<&Morpheme> {
        self.unknown_vocabulary.get(wid)
    }

    pub fn get_unknown_words_by_class(&self, class: &str) -> Vec<(usize, &Morpheme)> {
        self.unknown_classes
            .get(class)
            .unwrap()
            .iter()
            .map(|wid| (*wid, self.unknown_vocabulary.get(wid).unwrap()))
            .collect::<Vec<_>>()
    }

    pub fn resolve_homonyms(&self, wid: usize) -> Option<&Vec<usize>> {
        self.homonyms.get(&wid)
    }

    pub fn transition_cost(&self, left: usize, right: usize) -> Option<&i16> {
        if let Some(rights) = self.matrix.get(left) {
            if let Some(cost) = rights.get(right) {
                return Some(cost);
            }
        }
        None
    }

    pub fn occurrence_cost(&self, wid: &usize) -> Option<i16> {
        self.get_known_word(wid).map(|w| w.cost)
    }

    pub fn shrink_to_wids(&mut self, wids: &Vec<usize>) {
        let set: HashSet<usize> = HashSet::from_iter(wids.iter().cloned());
        self.homonyms.retain(|k, _| set.contains(k));
    }
}
