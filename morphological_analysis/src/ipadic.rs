use super::vocabulary::{ConjugationCategory, LexicalCategory, Word};
use encoding_rs::EUC_JP;
use glob::glob;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

type IPADictionaryLine = (
    String, // 表層形
    usize,  // 左文脈ID
    usize,  // 右文脈ID
    i16,    // コスト
    String, // 品詞
    String, // 品詞細分類1
    String, // 品詞細分類2
    String, // 品詞細分類3
    String, // 活用型
    String, // 活用形
    String, // 原形
    String, // 読み
    String, // 発音
);

pub fn load_dir(dir: &String) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut words = vec![];
    // TODO: It's for debug
    // let buff = PathBuf::from(dir).join("Filler.csv");
    let buff = PathBuf::from(dir).join("*.csv");
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
    for result in rdr.deserialize() {
        let (
            surface_form,
            left_context_id,
            right_context_id,
            cost,
            lexical_category,
            lexical_subcategory1,
            lexical_subcategory2,
            lexical_subcategory3,
            conjugation_category,
            conjugation,
            infinitive,
            ruby,
            pronounciation,
        ): IPADictionaryLine = result?;
        let lexical_category = match lexical_category.as_str() {
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
        let lexical_subcategory1 = wrap_value(lexical_subcategory1);
        let lexical_subcategory2 = wrap_value(lexical_subcategory2);
        let lexical_subcategory3 = wrap_value(lexical_subcategory3);
        let conjugation = wrap_value(conjugation);
        let conjugation_category = match wrap_value(conjugation_category) {
            None => None,
            _ => Some(ConjugationCategory::A),
        };
        words.push(Word {
            surface_form,
            left_context_id,
            right_context_id,
            cost,
            lexical_category,
            lexical_subcategory1,
            lexical_subcategory2,
            lexical_subcategory3,
            conjugation_category,
            conjugation,
            infinitive,
            ruby,
            pronounciation,
        })
    }
    Ok(words)
}

fn wrap_value(val: String) -> Option<String> {
    match val.as_str() {
        "*" => None,
        s => Some(s.to_string()),
    }
}
