use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const CLASS_DEFAULT: &str = "DEFAULT";

#[derive(
    Debug, PartialEq, Eq, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub enum InvokeTiming {
    Fallback,
    Always,
}
#[derive(
    Debug, PartialEq, Eq, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct CharDefinition {
    pub class: String,
    pub timing: InvokeTiming,
    pub group_by_same_kind: bool,
    pub len: usize,
    pub compatibilities: HashSet<String>, // elements = class name
}
impl CharDefinition {
    pub fn compatible_with(&self, class_name: &str) -> bool {
        self.class.eq(class_name) || self.compatibilities.contains(class_name)
    }
}

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct CharClass {
    range: (u32, u32),
    class: String,
}
impl CharClass {
    pub fn from(range: (u32, u32), class: String) -> CharClass {
        CharClass { range, class }
    }

    pub fn in_range(&self, c: &char) -> bool {
        let code = *c as u32;
        self.range.0 <= code && code <= self.range.1
    }
}

#[derive(Debug, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct CharClassifier {
    chars: HashMap<String, CharDefinition>,
    ranges: Vec<CharClass>,
}
impl CharClassifier {
    pub fn from(chars: HashMap<String, CharDefinition>, ranges: Vec<CharClass>) -> CharClassifier {
        CharClassifier { chars, ranges }
    }

    pub fn classify(&self, c: &char) -> &CharDefinition {
        let class = self.get_class_name(c);
        self.chars.get(class).unwrap()
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
                if def.len != 0 && len >= def.len || !def.compatible_with(self.get_class_name(c)) {
                    return false;
                }
                len += 1;
                true
            })
            .map(|(_, c)| c)
            .collect()
    }

    fn get_class_name(&self, c: &char) -> &str {
        self.ranges
            .iter()
            .find(|class| class.in_range(c))
            .map(|class| class.class.as_str())
            .unwrap_or_else(|| CLASS_DEFAULT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_with_without_compatibilities() {
        let def_a = CharDefinition {
            class: String::from("A"),
            timing: InvokeTiming::Always,
            group_by_same_kind: false,
            len: 2,
            compatibilities: HashSet::new(),
        };
        assert_eq!(def_a.compatible_with("A"), true);
        assert_eq!(def_a.compatible_with("B"), false);
    }

    #[test]
    fn compatible_with_with_compatibilities() {
        let mut compatibilities = HashSet::new();
        compatibilities.insert(String::from("B"));
        let def_a = CharDefinition {
            class: String::from("A"),
            timing: InvokeTiming::Always,
            group_by_same_kind: false,
            len: 2,
            compatibilities,
        };
        assert_eq!(def_a.compatible_with("A"), true);
        assert_eq!(def_a.compatible_with("B"), true);
        assert_eq!(def_a.compatible_with("C"), false);
    }

    #[test]
    fn in_range() {
        let class = CharClass::from((1, 2), String::new());
        assert_eq!(class.in_range(&(0 as char)), false);
        assert_eq!(class.in_range(&(1 as char)), true);
        assert_eq!(class.in_range(&(2 as char)), true);
        assert_eq!(class.in_range(&(3 as char)), false);
    }
}
