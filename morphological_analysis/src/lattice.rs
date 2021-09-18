use super::double_array::{DoubleArray, INDEX_ROOT};
use super::ipadic::IPADic;
use std::collections::{HashMap, HashSet, VecDeque};

const BOS_CONTEXT_ID: usize = 0;
const EOS_CONTEXT_ID: usize = 0;

#[derive(Debug)]
pub struct LatticeNode {
    // (word ID, length)
    begin_nodes: Vec<(usize, usize)>,
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
    pub fn parse(text: &str, da: &DoubleArray) -> Lattice {
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

            if let Ok((mut cursor, _)) = da.transition(INDEX_ROOT, c) {
                if let Ok(wid) = da.stop(cursor as usize) {
                    open_indices.push_back(index + 1);
                    indices[index].begin_nodes.push((wid, 1));
                }
                let mut j = index + 1;
                while j < len {
                    let c = text.chars().nth(j).unwrap();
                    match da.transition(cursor as usize, c) {
                        Ok((next, _)) => {
                            if let Ok(wid) = da.stop(next as usize) {
                                open_indices.push_back(j + 1 - index);
                                indices[index].begin_nodes.push((wid, j + 1 - index));
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
        }

        Lattice { indices }
    }

    // TODO: Move it into exporter_dot.rs
    pub fn as_dot(&self, dict: &IPADic) -> String {
        let dp = self.get_dp_table(dict);
        let best_path = self.find_best(dict);
        let len = self.indices.len();
        let mut dot = String::from("");
        dot.push_str("digraph lattice {\n");
        dot.push_str("  labelloc=\"t\";\n");
        dot.push_str("  label=\"N = gross min, (N) = individual cost\";\n");
        dot.push_str("  BOS [label=\"BOS\\n0 (0)\" shape=\"doublecircle\"];\n");
        dot.push_str("  EOS [label=\"EOS\\n(0)\" shape=\"doublecircle\"];\n");
        for (i, index) in self.indices.iter().enumerate() {
            for (left_id, wlen) in index.begin_nodes.iter() {
                let left = dict.get(left_id).unwrap();
                dot.push_str(&format!(
                    "  {} [label=\"{}\\n{} ({})\"{}];\n",
                    left_id,
                    dict.get(left_id).unwrap().surface_form,
                    dp.get(left_id).unwrap().0,
                    left.cost,
                    if best_path.contains(left_id) {
                        " shape=\"doublecircle\""
                    } else {
                        ""
                    }
                ));
                if i + wlen >= len {
                    let cost = dict
                        .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                        .unwrap();
                    dot.push_str(&format!("  {} -> EOS [label=\"({})\"];\n", left_id, cost));
                    continue;
                }
                if i == 0 {
                    let right = left;
                    let cost = dict
                        .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
                        .unwrap();
                    dot.push_str(&format!("  BOS -> {} [label=\"({})\"];\n", left_id, cost));
                }
                for (right_id, _) in self.indices[i + wlen].begin_nodes.iter() {
                    let right = dict.get(right_id).unwrap();
                    let cost = dict
                        .transition_cost(left.left_context_id, right.right_context_id)
                        .unwrap();
                    dot.push_str(&format!(
                        "  {} -> {} [label=\"({})\"];\n",
                        left_id, right_id, cost
                    ));
                }
            }
        }
        dot.push_str("}\n");
        dot
    }

    pub fn find_best(&self, dict: &IPADic) -> Vec<usize> {
        let dp = self.get_dp_table(dict);
        let mut path = vec![];
        let mut cursor = 0;
        while let Some((_, prev_wid)) = dp.get(&cursor) {
            if *prev_wid == 0 {
                break;
            }
            path.insert(0, *prev_wid);
            cursor = *prev_wid;
        }

        path
    }

    fn get_dp_table(&self, dict: &IPADic) -> HashMap<usize, (i32, usize)> {
        let len = self.indices.len();
        // word ID -> (min cost, prev)
        // 0 means BOS or EOS
        // individual cost should be less in i16, the sum of costs can exceed its range.
        let mut dp: HashMap<usize, (i32, usize)> = HashMap::new();

        for (right_id, _) in self.indices[0].begin_nodes.iter() {
            let right = dict.get(right_id).unwrap();
            let cost = dict
                .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
                .unwrap()
                + right.cost;
            dp.insert(*right_id, (cost as i32, 0));
        }

        for (i, index) in self.indices.iter().enumerate() {
            for (left_id, wlen) in index.begin_nodes.iter() {
                let left = dict.get(left_id).unwrap();
                let left_min = dp.get(left_id).unwrap().0;
                if i + wlen >= len {
                    let cost: i32 = (*dict
                        .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                        .unwrap() as i32)
                        + (left.cost as i32)
                        + left_min;
                    let entry = dp.entry(0).or_insert((i32::MAX, *left_id));
                    if cost < entry.0 {
                        entry.0 = cost as i32;
                        entry.1 = *left_id;
                    }
                    continue;
                }

                for (right_id, _) in self.indices[i + wlen].begin_nodes.iter() {
                    let right = dict.get(right_id).unwrap();
                    let cost: i32 = (*dict
                        .transition_cost(left.left_context_id, right.right_context_id)
                        .unwrap() as i32)
                        + left.cost as i32
                        + right.cost as i32
                        + left_min;
                    let entry = dp.entry(*right_id).or_insert((i32::MAX, *right_id));
                    if cost < entry.0 as i32 {
                        entry.0 = cost as i32;
                        entry.1 = *left_id;
                    }
                }
            }
        }
        dp
    }
}
