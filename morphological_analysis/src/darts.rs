use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Darts {
    pub codes: IndexSet<char>,
    pub base: Vec<i32>,
    pub check: Vec<usize>,
}

impl Darts {}
