use super::double_array::DoubleArray;
use super::ipadic::{CharDefinition, IPADic, InvokeTiming, WordIdentifier};
use std::collections::{HashSet, VecDeque};

const BOS_CONTEXT_ID: usize = 0;
const EOS_CONTEXT_ID: usize = 0;
const NODE_BOS: usize = 0;

#[derive(Debug)]
pub struct Lattice {
    indices: Vec<Vec<(WordIdentifier, usize)>>,
    dp: Vec<Vec<(i32, usize, usize)>>,
}
impl Lattice {
    pub fn parse(text: &str, da: &DoubleArray, dict: &IPADic) -> Lattice {
        let len = text.chars().count();
        let mut indices: Vec<Vec<(WordIdentifier, usize)>> = vec![vec![]; len];
        let mut open_indices = VecDeque::from(vec![0]);
        let mut visited = HashSet::with_capacity(len);
        let char_defs = text
            .chars()
            .map(|c| dict.get_char_def(c))
            .collect::<Vec<&CharDefinition>>();

        while let Some(index) = open_indices.pop_front() {
            if visited.contains(&index) || index >= len {
                continue;
            }
            visited.insert(index);

            let c = text.chars().nth(index).unwrap();
            let def = char_defs[index];
            if let InvokeTiming::Always = def.timing {
                let surface_form = dict.take_unknown_chars(def, text, index);
                open_indices.push_back(index + surface_form.chars().count());
                for (wid, _) in dict.get_unknown_words_by_class(&def.class) {
                    indices[index].push((
                        WordIdentifier::Unknown(wid, surface_form.to_string()),
                        surface_form.chars().count(),
                    ));
                }
            }

            match da.init(c) {
                Ok((mut cursor, _)) => {
                    match da.stop(cursor as usize) {
                        Ok(wid) => {
                            open_indices.push_back(index + 1);
                            for wid in dict.resolve_homonyms(wid).unwrap().iter() {
                                indices[index].push((WordIdentifier::Known(*wid), 1));
                            }
                        }
                        Err(_) => continue,
                    }
                    let mut j = index + 1;
                    while j < len {
                        let c = text.chars().nth(j).unwrap();
                        match da.transition(cursor as usize, c) {
                            Ok((next, _)) => {
                                match da.stop(next as usize) {
                                    Ok(wid) => {
                                        open_indices.push_back(j + 1);
                                        for wid in dict.resolve_homonyms(wid).unwrap().iter() {
                                            indices[index]
                                                .push((WordIdentifier::Known(*wid), j + 1 - index));
                                        }
                                    }
                                    Err(_) => continue,
                                }
                                cursor = next;
                            }
                            Err(_) => break,
                        }
                        j += 1;
                    }
                }
                Err(_) => {
                    if let InvokeTiming::Fallback = def.timing {
                        let surface_form = dict.take_unknown_chars(def, text, index);
                        open_indices.push_back(index + surface_form.chars().count());
                        for (wid, _) in dict.get_unknown_words_by_class(&def.class) {
                            indices[index].push((
                                WordIdentifier::Unknown(wid, surface_form.to_string()),
                                surface_form.chars().count(),
                            ));
                        }
                    }
                }
            }
        }

        Lattice {
            dp: get_dp_table(&indices, dict),
            indices,
        }
    }

    pub fn word_identifiers(&self) -> Vec<WordIdentifier> {
        let mut wids = vec![];
        for idx in self.indices.iter() {
            for (wid, _) in idx.iter() {
                wids.push(wid.clone())
            }
        }
        wids
    }

    // FIXME: This is not a concern of this struct
    pub fn as_dot(&self, dict: &IPADic) -> String {
        let bold = " penwidth=3";
        let len = self.indices.len();
        let best = self.find_best_path();
        let mut dot = String::from("");
        dot.push_str("digraph lattice {\n");
        dot.push_str("  labelloc=\"t\";\n");
        dot.push_str("  label=\"N = gross min, (N) = individual cost\";\n");
        dot.push_str("  BOS [label=\"BOS\\n0 (0)\" shape=\"doublecircle\"];\n");
        dot.push_str("  EOS [label=\"EOS\\n(0)\" shape=\"doublecircle\"];\n");
        for (i, index) in self.indices.iter().enumerate() {
            for (j, (left_wid, wlen)) in index.iter().enumerate() {
                let left = dict.get_word(left_wid).unwrap();
                let node_style = match &best {
                    Some(best) if best.contains(&(i + 1, j)) => bold,
                    _ => "",
                };
                dot.push_str(&format!(
                    "  \"{}_{}\" [label=\"{}\\n{} ({}, {})\"{}];\n",
                    i,
                    j,
                    left.surface_form,
                    left.pos().unwrap(),
                    self.dp[i + 1][j].0,
                    left.cost,
                    node_style,
                ));
                if i == 0 {
                    let right = left;
                    let cost = dict
                        .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
                        .unwrap();
                    let bos_edge_style = match &best {
                        Some(best) if best.contains(&(i + 1, j)) => bold,
                        _ => "",
                    };
                    dot.push_str(&format!(
                        "  BOS -> \"{}_{}\" [label=\"({})\"{}];\n",
                        i, j, cost, bos_edge_style
                    ));
                }
                if i + wlen >= len {
                    let cost = dict
                        .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                        .unwrap();
                    let eos_edge_style = match &best {
                        Some(best) if best.contains(&(i + 1, j)) => bold,
                        _ => "",
                    };
                    dot.push_str(&format!(
                        "  \"{}_{}\" -> EOS [label=\"({})\"{}];\n",
                        i, j, cost, eos_edge_style
                    ));
                    continue;
                }
                for (k, (right_wid, _)) in self.indices[i + wlen].iter().enumerate() {
                    let right = dict.get_word(right_wid).unwrap();
                    let cost = dict
                        .transition_cost(left.left_context_id, right.right_context_id)
                        .unwrap();
                    let edge_style = match &best {
                        Some(best)
                            if best.contains(&(i + 1, j)) && best.contains(&(i + wlen + 1, k)) =>
                        {
                            bold
                        }
                        _ => "",
                    };
                    dot.push_str(&format!(
                        "  \"{}_{}\" -> \"{}_{}\" [label=\"({})\"{}];\n",
                        i,
                        j,
                        i + wlen,
                        k,
                        cost,
                        edge_style
                    ));
                }
            }
        }
        dot.push_str("}\n");
        dot
    }

    fn find_best_path(&self) -> Option<Vec<(usize, usize)>> {
        let mut path = vec![];
        let mut cursor = (self.dp.len() - 1, 0);
        loop {
            match self.dp[cursor.0].get(cursor.1) {
                Some((_, i, j)) => {
                    if *i == NODE_BOS {
                        break;
                    }
                    path.insert(0, (*i, *j));
                    cursor = (*i, *j);
                }
                _ => return None,
            }
        }
        Some(path)
    }

    pub fn find_best(&self) -> Option<Vec<WordIdentifier>> {
        match self.find_best_path() {
            Some(best_path) => {
                let mut ids = vec![];
                for (i, j) in best_path.iter() {
                    ids.push(self.indices[*i - 1][*j].0.clone());
                }
                Some(ids)
            }
            None => None,
        }
    }
}

fn get_dp_table(
    indices: &[Vec<(WordIdentifier, usize)>],
    dict: &IPADic,
) -> Vec<Vec<(i32, usize, usize)>> {
    let len = indices.len();
    let max_num_childs = indices.iter().map(|idx| idx.len()).max().unwrap();
    // (min cost, idx of indices, idx2 of indices[idx])
    // * dp[0][0] means BOS
    // * dp[dp.len() - 1][0] means EOS
    // Individual cost should be less in i16, the sum of costs can exceed its range.
    // Currently each element has unused indices to reduce num alloc
    let mut dp: Vec<Vec<(i32, usize, usize)>> =
        vec![vec![(i32::MAX, 0, 0); max_num_childs]; len + 2];
    if max_num_childs == 0 {
        return dp;
    }
    dp[0][0] = (0, 0, 0);

    for (i, (right_wid, _)) in indices[0].iter().enumerate() {
        let right = dict.get_word(right_wid).unwrap();
        let cost = dict
            .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
            .unwrap()
            + right.cost;
        dp[1][i] = (cost as i32, NODE_BOS, 0);
    }

    for (i, index) in indices.iter().enumerate() {
        for (j, (left_wid, wlen)) in index.iter().enumerate() {
            let before_cost = dp[i + 1][j].0;
            let left = dict.get_word(left_wid).unwrap();
            if i + wlen >= len {
                let cost = (*dict
                    .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                    .unwrap() as i32)
                    + (left.cost as i32)
                    + before_cost;
                if cost < dp[i + wlen + 1][0].0 {
                    dp[i + wlen + 1][0] = (cost, i + 1, j);
                }
                continue;
            }

            for (k, (right_wid, _)) in indices[i + wlen].iter().enumerate() {
                let right = dict.get_word(right_wid).unwrap();
                let cost = (*dict
                    .transition_cost(left.left_context_id, right.right_context_id)
                    .unwrap() as i32)
                    + left.cost as i32
                    + right.cost as i32
                    + before_cost;
                if cost < dp[i + 1 + wlen][k].0 {
                    dp[i + 1 + wlen][k] = (cost, i + 1, j);
                }
            }
        }
    }
    dp
}
