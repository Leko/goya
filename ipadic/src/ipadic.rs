use goya::char_class::CharClassifier;
use goya::char_class::CharDefinition;
use goya::dictionary::Dictionary;
use goya::morpheme::Morpheme;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::vec::Vec;

// TODO: Make it newtype idiom
type MorphemeId = usize;
#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct IPADic {
    vocabulary: HashMap<usize, Morpheme>,
    homonyms: HashMap<MorphemeId, Vec<usize>>,
    // FIXME: Remove
    classes: CharClassifier,
    matrix: Vec<Vec<i16>>,
    /// 1つのカテゴリに複数の素性を定義してもかまいません. 学習後, 適切なコスト値が 自動的に与えられます.
    /// https://taku910.github.io/mecab/learn.html#config
    unknown_classes: HashMap<String, Vec<usize>>,
    unknown_vocabulary: HashMap<usize, Morpheme>,
}
impl Dictionary for IPADic {
    fn get_known_morpheme(&self, wid: &usize) -> Option<&Morpheme> {
        self.vocabulary.get(wid)
    }

    fn get_unknown_morpheme(&self, wid: &usize) -> Option<&Morpheme> {
        self.unknown_vocabulary.get(wid)
    }

    fn resolve_homonyms(&self, wid: &usize) -> Option<&Vec<usize>> {
        self.homonyms.get(wid)
    }

    fn take_unknown_chars_seq(&self, def: &CharDefinition, text: &str, start: &usize) -> String {
        self.classes.take_unknown_chars(def, text, start)
    }

    fn classify_char(&self, c: &char) -> &CharDefinition {
        self.classes.classify(*c)
    }

    fn get_unknown_morphemes_by_class(&self, class: &str) -> Vec<(usize, &Morpheme)> {
        self.unknown_classes
            .get(class)
            .unwrap()
            .iter()
            .map(|wid| (*wid, self.unknown_vocabulary.get(wid).unwrap()))
            .collect::<Vec<_>>()
    }

    fn transition_cost(&self, left: &usize, right: &usize) -> Option<&i16> {
        if let Some(rights) = self.matrix.get(*left) {
            if let Some(cost) = rights.get(*right) {
                return Some(cost);
            }
        }
        None
    }

    fn occurrence_cost(&self, wid: &usize) -> Option<i16> {
        self.get_known_morpheme(wid).map(|w| w.cost)
    }
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

    pub fn shrink_to_wids(&mut self, wids: &Vec<usize>) {
        let set: HashSet<usize> = HashSet::from_iter(wids.iter().cloned());
        self.homonyms.retain(|k, _| set.contains(k));
    }
}
