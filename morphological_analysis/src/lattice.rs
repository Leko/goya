use super::double_array::{DoubleArray, INDEX_ROOT};
use super::ipadic::{IPADic, InvokeTiming, WordIdentifier};
use std::collections::{HashSet, VecDeque};

const BOS_CONTEXT_ID: usize = 0;
const EOS_CONTEXT_ID: usize = 0;
const NODE_BOS: usize = 0;

#[derive(Debug)]
pub struct LatticeNode {
    /// (word ID, word char count)
    begin_nodes: Vec<(WordIdentifier, usize)>,
}
impl LatticeNode {
    pub fn new() -> LatticeNode {
        LatticeNode {
            begin_nodes: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Lattice {
    indices: Vec<LatticeNode>,
}
impl Lattice {
    pub fn parse(text: &str, da: &DoubleArray, dict: &IPADic) -> Lattice {
        let len = text.chars().count();
        let mut indices = text
            .char_indices()
            .map(|_| LatticeNode::new())
            .collect::<Vec<_>>();
        let mut open_indices = VecDeque::from(vec![0]);
        let mut visited = HashSet::with_capacity(len);
        while let Some(index) = open_indices.pop_front() {
            if visited.contains(&index) || index >= len {
                continue;
            }
            visited.insert(index);

            let c = text.chars().nth(index).unwrap();
            let class = dict.get_char_class(c);
            let def = dict.get_char_def(class);
            if let InvokeTiming::Always = def.timing {
                for (wid, _) in dict.get_unknown_words_by_class(&class.to_string()).iter() {
                    indices[index]
                        .begin_nodes
                        .push((WordIdentifier::Unknown(*wid), def.len));
                }
            }

            match da.transition(INDEX_ROOT, c) {
                Ok((mut cursor, _)) => {
                    if let Ok(wid) = da.stop(cursor as usize) {
                        open_indices.push_back(index + 1);
                        indices[index]
                            .begin_nodes
                            .push((WordIdentifier::Known(wid), 1));
                    }
                    let mut j = index + 1;
                    while j < len {
                        let c = text.chars().nth(j).unwrap();
                        match da.transition(cursor as usize, c) {
                            Ok((next, _)) => {
                                if let Ok(wid) = da.stop(next as usize) {
                                    open_indices.push_back(j + 1 - index);
                                    indices[index]
                                        .begin_nodes
                                        .push((WordIdentifier::Known(wid), j + 1 - index));
                                }
                                cursor = next;
                            }
                            Err(_) => {
                                break;
                            }
                        }
                        j += 1;
                    }
                }
                _ => {
                    // TODO: Handle unknown word
                }
            }
        }

        Lattice { indices }
    }

    // FIXME: This is not a concern of this struct
    pub fn as_dot(&self, dict: &IPADic) -> String {
        let dp = self.get_dp_table(dict);
        let len = self.indices.len();
        let mut dot = String::from("");
        dot.push_str("digraph lattice {\n");
        dot.push_str("  labelloc=\"t\";\n");
        dot.push_str("  label=\"N = gross min, (N) = individual cost\";\n");
        dot.push_str("  BOS [label=\"BOS\\n0 (0)\" shape=\"doublecircle\"];\n");
        dot.push_str("  EOS [label=\"EOS\\n(0)\" shape=\"doublecircle\"];\n");
        for (i, index) in self.indices.iter().enumerate() {
            for (j, (left_wid, wlen)) in index.begin_nodes.iter().enumerate() {
                let left = dict.get_word(left_wid).unwrap();
                dot.push_str(&format!(
                    "  \"{}_{}\" [label=\"{}\\n{} ({})\"];\n",
                    i,
                    j,
                    left.surface_form,
                    dp[i + 1][j].0,
                    left.cost,
                ));
                if i + wlen >= len {
                    let cost = dict
                        .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                        .unwrap();
                    dot.push_str(&format!(
                        "  \"{}_{}\" -> EOS [label=\"({})\"];\n",
                        i, j, cost
                    ));
                    continue;
                }
                if i == 0 {
                    let right = left;
                    let cost = dict
                        .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
                        .unwrap();
                    dot.push_str(&format!(
                        "  BOS -> \"{}_{}\" [label=\"({})\"];\n",
                        i, j, cost
                    ));
                }
                for (k, (right_wid, _)) in self.indices[i + wlen].begin_nodes.iter().enumerate() {
                    let right = dict.get_word(right_wid).unwrap();
                    let cost = dict
                        .transition_cost(left.left_context_id, right.right_context_id)
                        .unwrap();
                    dot.push_str(&format!(
                        "  \"{}_{}\" -> \"{}_{}\" [label=\"({})\"];\n",
                        i,
                        j,
                        i + wlen,
                        k,
                        cost
                    ));
                }
            }
        }
        dot.push_str("}\n");
        dot
    }

    pub fn find_best(&self, dict: &IPADic) -> Vec<WordIdentifier> {
        let dp = self.get_dp_table(dict);
        let mut path = vec![];
        let mut cursor = (dp.len() - 1, 0);
        loop {
            let (_, i, j) = dp[cursor.0][cursor.1];
            if i == NODE_BOS {
                break;
            }
            // FIXME: Replace it with Copy trait
            let id = match self.indices[i - 1].begin_nodes[j].0 {
                WordIdentifier::Known(wid) => WordIdentifier::Known(wid),
                WordIdentifier::Unknown(wid) => WordIdentifier::Unknown(wid),
            };
            path.insert(0, id);
            cursor = (i, j);
        }
        path
    }

    fn get_dp_table(&self, dict: &IPADic) -> Vec<Vec<(i32, usize, usize)>> {
        let len = self.indices.len();
        let max_num_childs = self
            .indices
            .iter()
            .map(|idx| idx.begin_nodes.len())
            .max()
            .unwrap();
        // (min cost, idx of indices, idx2 of indices[idx])
        // * dp[0][0] means BOS
        // * dp[dp.len() - 1][0] means EOS
        // Individual cost should be less in i16, the sum of costs can exceed its range.
        // Currently each element has unused indices to reduce num alloc
        let mut dp: Vec<Vec<(i32, usize, usize)>> =
            vec![vec![(i32::MAX, 0, 0); max_num_childs]; len + 2];
        dp[0][0] = (0, 0, 0);

        for (i, (right_wid, _)) in self.indices[0].begin_nodes.iter().enumerate() {
            let right = dict.get_word(right_wid).unwrap();
            let cost = dict
                .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
                .unwrap()
                + right.cost;
            dp[1][i] = (cost as i32, NODE_BOS, 0);
        }

        for (i, index) in self.indices.iter().enumerate() {
            for (j, (left_wid, wlen)) in index.begin_nodes.iter().enumerate() {
                let before_cost = dp[i + 1][j].0;
                let left = dict.get_word(left_wid).unwrap();
                if i + wlen >= len {
                    let cost: i32 = (*dict
                        .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                        .unwrap() as i32)
                        + (left.cost as i32)
                        + before_cost;
                    if cost < dp[i + wlen + 1][0].0 {
                        dp[i + wlen + 1][0] = (cost as i32, i + 1, j);
                    }
                    continue;
                }

                for (k, (right_wid, _)) in self.indices[i + wlen].begin_nodes.iter().enumerate() {
                    let right = dict.get_word(right_wid).unwrap();
                    let cost: i32 = (*dict
                        .transition_cost(left.left_context_id, right.right_context_id)
                        .unwrap() as i32)
                        + left.cost as i32
                        + right.cost as i32
                        + before_cost;
                    if cost < dp[i + 1 + wlen][k].0 {
                        dp[i + 1 + wlen][k] = (cost as i32, i + 1, j);
                    }
                }
            }
        }
        dp
    }
}
