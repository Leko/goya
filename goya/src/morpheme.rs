use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Clone,
    Serialize,
    Deserialize,
    rkyv::Archive,
    rkyv::Serialize,
    rkyv::Deserialize,
)]
pub struct Morpheme {
    /// 左文脈ID (単語を左から見たときの文脈 ID)
    /// https://taku910.github.io/mecab/dic-detail.html
    pub left_context_id: usize,
    /// 右文脈ID (単語を右から見たときの文脈 ID)
    /// https://taku910.github.io/mecab/dic-detail.html
    pub right_context_id: usize,
    /// > 単語コスト (小さいほど出現しやすい)
    /// > コスト値は short int (16bit 整数) の範囲におさめる必要があります.
    pub cost: i16,
}
impl Morpheme {
    pub fn new(left_context_id: usize, right_context_id: usize, cost: i16) -> Morpheme {
        Morpheme {
            left_context_id,
            right_context_id,
            cost,
        }
    }
}
