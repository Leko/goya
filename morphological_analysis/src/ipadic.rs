use super::vocabulary::{ConjugationCategory, LexicalCategory, Word};
use encoding_rs::EUC_JP;
use glob::glob;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
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

pub fn load_dir(dir: &String) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut words = vec![];
    let buff = PathBuf::from(dir).join("Filler.csv");
    // let buff = PathBuf::from(dir).join("*.csv");
    let pattern = buff.to_str().expect("Failed to build a glob pattern");
    for path in glob(pattern)? {
        words.append(&mut load_file(path?)?);
    }
    Ok(words)
}

fn load_file(path: PathBuf) -> Result<Vec<Word>, Box<dyn Error>> {
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
        let conjugation_category = match wrap_value(&row[COL_CONJUGATION_CATEGORY]) {
            None => None,
            _ => Some(ConjugationCategory::A),
        };
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

fn wrap_value(val: &str) -> Option<String> {
    match val {
        "*" => None,
        s => Some(s.to_string()),
    }
}
