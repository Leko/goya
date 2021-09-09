use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct MatchResult {
    pub next_state: usize,
    pub can_stop: bool,
}

#[derive(Debug)]
pub struct DoubleArray {
    codes: HashMap<char, usize>,
    base: Vec<i32>,
    check: Vec<usize>,
}
impl DoubleArray {
    pub fn new(base: Vec<i32>, check: Vec<usize>, codes: HashMap<char, usize>) -> DoubleArray {
        DoubleArray { base, check, codes }
    }

    pub fn from() -> DoubleArray {
        let base: Vec<i32> = vec![];
        let check: Vec<usize> = vec![];
        let codes: HashMap<char, usize> = HashMap::new();
        DoubleArray::new(base, check, codes)
    }

    pub fn match_char(&self, s: usize, c: &char) -> Option<MatchResult> {
        if self.base[s] < 0 {
            return None;
        }
        match self.next(s, c) {
            Some(t) => match self.check[t] == s {
                true => Some(MatchResult {
                    next_state: t,
                    can_stop: self.match_char(t, &'\0').is_some(),
                }),
                _ => None,
            },
            None => None,
        }
    }

    fn next(&self, s: usize, c: &char) -> Option<usize> {
        match self.codes.get(c) {
            Some(code) => {
                let next = self.base[s] + *code as i32;
                if next < 0 {
                    return None;
                }
                Some((next).try_into().unwrap())
            }
            None => None,
        }
    }
}
