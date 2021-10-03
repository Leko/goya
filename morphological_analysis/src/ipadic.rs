use super::vocabulary::Word;
use encoding_rs::EUC_JP;
use glob::glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::ops::RangeInclusive;
use std::path::Path;
use std::vec::Vec;
use std::{fs, vec};

const COL_SURFACE_FORM: usize = 0; // 表層形
const COL_LEFT_CONTEXT_ID: usize = 1; // 左文脈ID
const COL_RIGHT_CONTEXT_ID: usize = 2; // 右文脈ID
const COL_COST: usize = 3; // コスト
const CLASS_DEFAULT: &str = "DEFAULT";

#[derive(Debug, Clone)]
pub enum WordIdentifier {
    Known(usize),
    Unknown(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InvokeTiming {
    Fallback,
    Always,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CharDefinition {
    pub timing: InvokeTiming,
    pub group_by_same_kind: bool,
    pub len: usize,
}
#[derive(Debug, Serialize, Deserialize)]
struct CharClass {
    range: RangeInclusive<char>,
    class: String,
    compatible_categories: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct CharClassifier {
    chars: HashMap<String, CharDefinition>,
    ranges: Vec<CharClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IPADic {
    pub vocabulary: HashMap<usize, Word>,
    homonyms: HashMap<String, Vec<usize>>,
    classes: CharClassifier,
    matrix: Vec<Vec<i16>>,
    /// 1つのカテゴリに複数の素性を定義してもかまいません. 学習後, 適切なコスト値が 自動的に与えられます.
    /// https://taku910.github.io/mecab/learn.html#config
    unknown_classes: HashMap<String, Vec<usize>>,
    unknown_vocabulary: HashMap<usize, Word>,
}
impl IPADic {
    pub fn from_dir(dir: &str) -> Result<IPADic, Box<dyn Error>> {
        let classes = load_chars(Path::new(dir).join("char.def"))?;
        let matrix = load_matrix(Path::new(dir).join("matrix.def"))?;
        let unknown = load_unknown(Path::new(dir).join("unk.def"))?;
        let csv_pattern = Path::new(dir).join("*.csv");
        let csv_pattern = csv_pattern.to_str().ok_or("Failed to build glob pattern")?;

        let mut vocabulary = HashMap::new();
        let mut homonyms = HashMap::new();
        let mut id = 1;
        for path in glob(csv_pattern)? {
            for word in load_words_csv(path?)? {
                homonyms
                    .entry(word.surface_form.to_string())
                    .or_insert_with(Vec::new)
                    .push(id);
                vocabulary.insert(id, word);
                id += 1;
            }
        }

        let mut unknown_vocabulary = HashMap::new();
        let mut unknown_classes = HashMap::new();
        let mut id = 1;
        for (class, words) in unknown.into_iter() {
            for word in words {
                unknown_vocabulary.insert(id, word);
                unknown_classes
                    .entry(class.to_string())
                    .or_insert_with(Vec::new)
                    .push(id);
                id += 1;
            }
        }
        Ok(IPADic {
            vocabulary,
            homonyms,
            classes,
            matrix,
            unknown_vocabulary,
            unknown_classes,
        })
    }

    // TODO: Create another struct
    pub fn shrink(&self, wids: Vec<WordIdentifier>) -> IPADic {
        let mut vocabulary = HashMap::new();
        let mut unknown_vocabulary = HashMap::new();

        for wid in wids.into_iter() {
            let word = self.get_word(&wid).unwrap();
            match wid {
                WordIdentifier::Known(id) => {
                    vocabulary.insert(id, word.clone());
                }
                WordIdentifier::Unknown(id) => {
                    unknown_vocabulary.insert(id, word.clone());
                }
            }
        }

        IPADic {
            vocabulary,
            homonyms: HashMap::new(),
            classes: CharClassifier {
                chars: HashMap::new(),
                ranges: vec![],
            },
            matrix: self.matrix.clone(),
            unknown_classes: HashMap::new(),
            unknown_vocabulary,
        }
    }

    pub fn get_word(&self, wid: &WordIdentifier) -> Option<&Word> {
        match wid {
            WordIdentifier::Known(wid) => self.get_known_word(wid),
            WordIdentifier::Unknown(wid) => self.get_unknown_word(wid),
        }
    }

    pub fn get_known_word(&self, wid: &usize) -> Option<&Word> {
        self.vocabulary.get(wid)
    }

    pub fn get_unknown_word(&self, wid: &usize) -> Option<&Word> {
        self.unknown_vocabulary.get(wid)
    }

    pub fn get_unknown_words_by_class(&self, class: &str) -> Vec<(usize, &Word)> {
        self.unknown_classes
            .get(class)
            .unwrap()
            .iter()
            .map(|wid| (*wid, self.unknown_vocabulary.get(wid).unwrap()))
            .collect::<Vec<_>>()
    }

    pub fn resolve_homonyms(&self, wid: usize) -> Option<&Vec<usize>> {
        if let Some(word) = self.get_known_word(&wid) {
            return Some(self.homonyms.get(&word.surface_form).unwrap());
        }
        None
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

    pub fn get_char_class(&self, c: char) -> &str {
        for class in self.classes.ranges.iter() {
            if class.range.contains(&c) {
                return &class.class;
            }
        }
        CLASS_DEFAULT
    }

    pub fn get_char_def(&self, class: &str) -> &CharDefinition {
        self.classes.chars.get(class).unwrap()
    }
}

fn load_words_csv<P>(path: P) -> Result<Vec<Word>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let mut rdr = csv::Reader::from_reader(utf8.as_bytes());
    let mut words = vec![];
    for row in rdr.records() {
        let row = row?;
        words.push(Word::new(
            row[COL_SURFACE_FORM].to_string(),
            row[COL_LEFT_CONTEXT_ID].parse::<usize>().unwrap(),
            row[COL_RIGHT_CONTEXT_ID].parse::<usize>().unwrap(),
            row[COL_COST].parse::<i16>().unwrap(),
            row.iter()
                .skip(COL_COST + 1)
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        ))
    }
    Ok(words)
}

fn load_chars<P>(path: P) -> Result<CharClassifier, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let lines = utf8
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| Regex::new(r"#.*$").unwrap().replace(line, ""))
        .collect::<Vec<_>>();

    let head = lines.iter().take_while(|line| {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        !parts[0].starts_with("0x")
    });
    let mut chars = HashMap::new();
    for line in head {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let kind = parts[0].to_owned();
        let timing = if parts[1] == "0" {
            InvokeTiming::Fallback
        } else {
            InvokeTiming::Always
        };
        let group_by_same_kind = parts[2] == "1";
        let len = parts[3].parse::<usize>()?;
        chars.insert(
            kind,
            CharDefinition {
                timing,
                group_by_same_kind,
                len,
            },
        );
    }

    let tail = lines.iter().skip_while(|line| {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        !parts[0].starts_with("0x")
    });
    let mut ranges = vec![];
    for line in tail {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let range = parts[0]
            .split("..")
            .map(|c| u32::from_str_radix(&c[2..], 16).unwrap())
            .map(|c| char::from_u32(c).unwrap())
            .collect::<Vec<_>>();
        let range = if range.len() > 1 {
            range[0]..=range[1]
        } else {
            range[0]..=range[0]
        };
        let class = parts[1];
        let compatible_categories = parts
            .iter()
            .skip(2)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        ranges.push(CharClass {
            range,
            class: class.to_string(),
            compatible_categories,
        });
    }

    Ok(CharClassifier { chars, ranges })
}

fn load_matrix<P>(path: P) -> Result<Vec<Vec<i16>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let mut lines = utf8.lines();
    let size = lines
        .next()
        .expect("failed to read the first line")
        .split_ascii_whitespace()
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut matrix = vec![vec![-1; size[1]]; size[0]];
    for line in lines {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let left = parts[0].parse::<usize>()?;
        let right = parts[1].parse::<usize>()?;
        let cost = parts[2].parse::<i16>()?;
        matrix[left][right] = cost;
    }
    Ok(matrix)
}

fn load_unknown<P>(path: P) -> Result<HashMap<String, Vec<Word>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let words = load_words_csv(path)?;
    let mut map = HashMap::<String, Vec<Word>>::new();
    for w in words.into_iter() {
        map.entry(w.surface_form.to_string())
            .or_insert_with(Vec::new)
            .push(w);
    }
    Ok(map)
}
