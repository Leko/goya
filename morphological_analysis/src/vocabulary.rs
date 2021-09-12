use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LexicalCategory {
    Adjective,
    Adnominal,
    Adverb,
    Auxil,
    Conjunction,
    Interjection,
    Noun,
    PostpositionalParticle,
    Prefix,
    Suffix,
    Symbol,
    Verb,
    Filler,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConjugationCategory {
    A,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Word {
    pub surface_form: String,                              // 表層形
    pub left_context_id: usize,                            // 左文脈ID
    pub right_context_id: usize,                           // 右文脈ID
    pub cost: i16,                                         // コスト
    pub lexical_category: LexicalCategory,                 // 品詞
    pub lexical_subcategory1: Option<String>,              // 品詞細分類1
    pub lexical_subcategory2: Option<String>,              // 品詞細分類2
    pub lexical_subcategory3: Option<String>,              // 品詞細分類3
    pub conjugation_category: Option<ConjugationCategory>, // 活用型
    pub conjugation: Option<String>,                       // 活用形
    pub infinitive: String,                                // 原形
    pub ruby: String,                                      // 読み
    pub pronounciation: String,                            // 発音
}
impl Word {
    pub fn new(
        surface_form: impl Into<String>,
        left_context_id: usize,
        right_context_id: usize,
        cost: i16,
        lexical_category: LexicalCategory,
        lexical_subcategory1: Option<String>,
        lexical_subcategory2: Option<String>,
        lexical_subcategory3: Option<String>,
        conjugation_category: Option<ConjugationCategory>,
        conjugation: Option<String>,
        infinitive: impl Into<String>,
        ruby: impl Into<String>,
        pronounciation: impl Into<String>,
    ) -> Word {
        Word {
            surface_form: surface_form.into(),
            left_context_id,
            right_context_id,
            cost,
            lexical_category,
            lexical_subcategory1: lexical_subcategory1,
            lexical_subcategory2: lexical_subcategory2,
            lexical_subcategory3: lexical_subcategory3,
            conjugation_category,
            conjugation: conjugation,
            infinitive: infinitive.into(),
            ruby: ruby.into(),
            pronounciation: pronounciation.into(),
        }
    }
}
