#[derive(Debug)]
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

#[derive(Debug)]
pub enum ConjugationCategory {
    A,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Vocabulary {
    words: Vec<Word>,
}

impl Vocabulary {
    pub fn new(words: Vec<Word>) -> Vocabulary {
        Vocabulary { words }
    }
}
