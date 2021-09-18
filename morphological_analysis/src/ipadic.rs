use super::vocabulary::{LexicalCategory, Word};
use encoding_rs::EUC_JP;
use glob::glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::RangeInclusive;
use std::path::Path;
use std::vec::Vec;

const COL_SURFACE_FORM: usize = 0; // 表層形
const COL_LEFT_CONTEXT_ID: usize = 1; // 左文脈ID
const COL_RIGHT_CONTEXT_ID: usize = 2; // 右文脈ID
const COL_COST: usize = 3; // コスト
const COL_LEXICAL_CATEGORY: usize = 4; // 品詞
const COL_LEXICAL_SUBCATEGORY1: usize = 5; // 品詞細分類1
const COL_LEXICAL_SUBCATEGORY2: usize = 6; // 品詞細分類2
const COL_LEXICAL_SUBCATEGORY3: usize = 7; // 品詞細分類3
const COL_CONJUGATION_CATEGORY: usize = 8; // 活用型
const COL_CONJUGATION: usize = 9; // 活用形
const COL_INFINITIVE: usize = 10; // 原形
const COL_RUBY: usize = 11; // 読み
const COL_PRONOUNCIATION: usize = 12; // 発音

#[derive(Debug, Serialize, Deserialize)]
enum OperationTiming {
    OnlyUnknown,
    Always,
}
#[derive(Debug, Serialize, Deserialize)]
struct CharDefinition {
    timing: OperationTiming,
    group_by_same_kind: bool,
    len: usize,
}
#[derive(Debug, Serialize, Deserialize)]
struct CharClass {
    range: RangeInclusive<char>,
    category: String,
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
    chars: CharClassifier,
    matrix: HashMap<(usize, usize), i16>,
}
impl IPADic {
    pub fn load_dir(dir: &String) -> Result<IPADic, Box<dyn Error>> {
        let chars = load_chars(Path::new(dir).join("char.def"))?;
        let matrix = load_matrix(Path::new(dir).join("matrix.def"))?;

        // let csv_pattern = base.join("Filler.csv");
        let csv_pattern = Path::new(dir).join("*.csv");
        let csv_pattern = csv_pattern
            .to_str()
            .ok_or("Failed to build a glob pattern")?;

        let mut vocabulary = HashMap::new();
        let mut id = 1;
        for path in glob(csv_pattern)? {
            for w in load_csv(path?)? {
                vocabulary.insert(id, w);
                id += 1;
            }
        }
        Ok(IPADic {
            vocabulary,
            chars,
            matrix,
        })
    }

    pub fn get(&self, wid: &usize) -> Option<&Word> {
        self.vocabulary.get(wid)
    }

    pub fn transition_cost(&self, left: usize, right: usize) -> Option<&i16> {
        self.matrix.get(&(left, right))
    }

    pub fn occurrence_cost(&self, wid: &usize) -> Option<i16> {
        match self.get(wid) {
            Some(w) => Some(w.cost),
            _ => None,
        }
    }
}

fn load_csv<P>(path: P) -> Result<Vec<Word>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let mut rdr = csv::Reader::from_reader(utf8.as_bytes());
    let mut words = vec![];
    for row in rdr.records() {
        let row = row?;
        let lexical_category = match &row[COL_LEXICAL_CATEGORY] {
            "フィラー" => LexicalCategory::Filler,
            "形容詞" => LexicalCategory::Adjective,
            "連体詞" => LexicalCategory::Adnominal,
            "副詞" => LexicalCategory::Adverb,
            "助動詞" => LexicalCategory::Auxil,
            "接続詞" => LexicalCategory::Conjunction,
            "感動詞" => LexicalCategory::Interjection,
            "名詞" => LexicalCategory::Noun,
            "助詞" => LexicalCategory::PostpositionalParticle,
            "接頭詞" => LexicalCategory::Prefix,
            "記号" => LexicalCategory::Symbol,
            "動詞" => LexicalCategory::Verb,
            "その他" => LexicalCategory::Unknown,
            c => panic!("Unexpected lexical category: {}", c),
        };
        let lexical_subcategory1 = wrap_value(&row[COL_LEXICAL_SUBCATEGORY1]);
        let lexical_subcategory2 = wrap_value(&row[COL_LEXICAL_SUBCATEGORY2]);
        let lexical_subcategory3 = wrap_value(&row[COL_LEXICAL_SUBCATEGORY3]);
        let conjugation = wrap_value(&row[COL_CONJUGATION]);
        let conjugation_category = wrap_value(&row[COL_CONJUGATION_CATEGORY]);
        words.push(Word::new(
            row[COL_SURFACE_FORM].to_string(),
            row[COL_LEFT_CONTEXT_ID].parse::<usize>().unwrap(),
            row[COL_RIGHT_CONTEXT_ID].parse::<usize>().unwrap(),
            row[COL_COST].parse::<i16>().unwrap(),
            lexical_category,
            lexical_subcategory1,
            lexical_subcategory2,
            lexical_subcategory3,
            conjugation_category,
            conjugation,
            row[COL_INFINITIVE].to_string(),
            row[COL_RUBY].to_string(),
            row[COL_PRONOUNCIATION].to_string(),
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
        .filter(|line| line.len() > 0 && !line.starts_with('#'))
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
            OperationTiming::OnlyUnknown
        } else {
            OperationTiming::Always
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
        let category = parts[1];
        let compatible_categories = parts
            .iter()
            .skip(2)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        ranges.push(CharClass {
            range,
            category: category.to_string(),
            compatible_categories,
        });
    }

    Ok(CharClassifier { chars, ranges })
}

fn load_matrix<P>(path: P) -> Result<HashMap<(usize, usize), i16>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let mut matrix = HashMap::new();
    let mut lines = utf8.lines();
    lines.next().ok_or("failed to read the first line")?;
    for line in lines {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let left = parts[0].parse::<usize>()?;
        let right = parts[1].parse::<usize>()?;
        let cost = parts[2].parse::<i16>()?;
        matrix.insert((left, right), cost);
    }
    Ok(matrix)
}

fn wrap_value(val: &str) -> Option<String> {
    match val {
        "*" => None,
        s => Some(s.to_string()),
    }
}
