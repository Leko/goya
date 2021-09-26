use std::collections::HashMap;

use itertools::Itertools;

// Direct Construction of Minimal Acyclic Subsequential Transducers, Stoyan Mihov and Denis Maurel, 2001.
// http://citeseerx.ist.psu.edu/viewdoc/download;jsessionid=CDB069E19B303A57134EF46F36F063FB?doi=10.1.1.24.3698&rep=rep1&type=pdf

const DEFAULT_SIZE: usize = 1024;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pair<In, Out> {
    input: In,
    output: Out,
}
impl Pair<String, usize> {
    pub fn from(input: String, output: usize) -> Pair<String, usize> {
        Pair { input, output }
    }

    pub fn common_prefix_len(&self, other: &String) -> usize {
        let self_len = self.input.chars().count();
        let other_len = other.chars().count();
        let mut end = self_len;
        if end > other_len {
            end = other_len;
        }
        let mut i = 0;
        while i < end && self.input.chars().nth(i) == other.chars().nth(i) {
            i += 1;
        }
        i
    }
}

#[derive(Debug)]
pub struct Pairs<In, Out> {
    pairs: Vec<Pair<In, Out>>,
}
impl Pairs<String, usize> {
    pub fn from(pairs: Vec<Pair<String, usize>>) -> Pairs<String, usize> {
        Pairs { pairs }
    }

    pub fn max_len(&self) -> Option<usize> {
        self.pairs.iter().map(|p| p.input.chars().count()).max()
    }

    pub fn iter(&self) -> std::vec::IntoIter<&Pair<String, usize>> {
        self.pairs.iter().sorted_by(|a, b| a.cmp(b))
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct State {
    id: usize,
    trans: HashMap<char, State>,
    tail: HashMap<char, bool>,
    is_final: bool,
    prev: Vec<State>,
    hcode: usize,
}
impl State {
    pub fn new() -> State {
        State::default()
    }

    pub fn set_transition(&mut self, c: char, next: State) {
        let next_id = next.id;
        self.trans.insert(c, next);

        let magic = 1001;
        self.hcode += (c as usize + next_id) * magic;
    }

    pub fn set_inv_transition(&mut self) {
        for next in self.trans.values_mut() {
            // FIXME: Copy
            next.prev.push(self.clone());
        }
    }
}

pub struct MAST {
    initial_state: State,
    states: Vec<State>,
    final_states: Vec<State>,
}

impl MAST {
    pub fn from(pairs: &mut Pairs<String, usize>) {
        // pairs.sort();

        // let mut mast = MAST {
        //     initial_state: State {},
        // };
        let mut dic: HashMap<usize, Vec<State>> = HashMap::new();
        let mut states: Vec<State> = Vec::with_capacity(DEFAULT_SIZE);
        let mut final_states: Vec<State> = Vec::with_capacity(DEFAULT_SIZE);

        let len = pairs.max_len().unwrap() + 1;
        let mut buf: Vec<State> = Vec::with_capacity(len);
        for i in 0..=len {
            buf[i] = State::new();
        }

        let mut prev = String::from("");
        for pair in pairs.iter() {
            let prefix_len = pair.common_prefix_len(&prev);
            let mut i = prev.chars().count();
            while i > prefix_len {
                let mut s: Option<State> = None;
                if let Some(cs) = dic.get(&buf[i].hcode) {
                    for c in cs.iter() {
                        if c == &buf[i] {
                            // FIXME: Copy
                            s = Some(c.clone());
                            break;
                        }
                    }
                }
                if s.is_none() {
                    s = Some(State::new());

                    // FIXME: What is the equavalent of Go
                    // *s = *buf[i];
                    // buf[i].renew();

                    dic.get_mut(&s.unwrap().hcode)
                        .unwrap()
                        .push(s.unwrap().clone());
                }
                buf[i - 1].set_transition(prev.chars().nth(i - 1).unwrap(), s.unwrap());
                s.unwrap().set_inv_transition();
                i -= 1;
            }
        }
        let mut i = prev.chars().count();
        while i > 0 {
            i -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_tiny() {
        let mut pairs = Pairs::from(vec![
            Pair::from("abc".to_string(), 1),
            Pair::from("abd".to_string(), 2),
        ]);
        MAST::from(&mut pairs);
    }
}
