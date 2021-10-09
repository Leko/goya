use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub enum InvokeTiming {
    Fallback,
    Always,
}
#[derive(
    Debug,
    PartialEq,
    Clone,
    Eq,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct CharDefinition {
    pub range: (u32, u32),
    pub class: String,
    pub timing: InvokeTiming,
    pub group_by_same_kind: bool,
    pub len: usize,
    pub compatibilities: HashSet<String>,
}
impl CharDefinition {
    pub fn compatible_with(&self, class_name: &str) -> bool {
        self.class.eq(class_name) || self.compatibilities.contains(class_name)
    }
}

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct CharClassifier {
    defs: Vec<CharDefinition>,
    default_def: CharDefinition,
}
impl CharClassifier {
    pub fn from(defs: Vec<CharDefinition>, default_def: CharDefinition) -> CharClassifier {
        CharClassifier { defs, default_def }
    }

    pub fn classify(&self, c: char) -> &CharDefinition {
        let code = c as u32;
        self.defs
            .iter()
            .find(|class| class.range.0 <= code && code <= class.range.1)
            .unwrap_or_else(|| &self.default_def)
    }

    pub fn take_unknown_chars(&self, def: &CharDefinition, text: &str, start: &usize) -> String {
        if !def.group_by_same_kind {
            return text.chars().skip(*start).take(def.len).collect();
        }

        let mut len = 0;
        text.chars()
            .enumerate()
            .skip(*start)
            .take_while(|(_, c)| {
                if def.len != 0 && len >= def.len || !def.compatible_with(&self.classify(*c).class)
                {
                    return false;
                }
                len += 1;
                true
            })
            .map(|(_, c)| c)
            .collect()
    }
}
