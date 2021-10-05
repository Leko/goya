use super::id::ID;

pub struct WordSet {
    known: Vec<Word>,
    unknown: Vec<Word>,
}
impl WordSet {}

#[derive(
    Debug, Clone, Serialize, Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct WordSurface {
    /// 表層形
    /// https://taku910.github.io/mecab/dic-detail.html
    pub surface_form: String,
    /// > 5カラム目以降は, ユーザ定義の CSV フィールドです. 基本的に どんな内容でも CSV の許す限り追加することができます.
    /// > https://taku910.github.io/mecab/dic-detail.html
    pub features: Vec<String>,
}
impl WordSurface {
    pub fn new(surface_form: impl Into<String>, features: Vec<String>) -> WordSurface {
        WordSurface {
            surface_form: surface_form.into(),
            features,
        }
    }

    // FIXME: Depends on IPADic
    pub fn pos(&self) -> Option<&String> {
        self.features.get(META_POS)
    }
}
