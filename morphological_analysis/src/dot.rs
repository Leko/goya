use std::{error::Error, fmt::Write};

use crate::{
    ipadic::IPADic,
    lattice::{Lattice, BOS_CONTEXT_ID, EOS_CONTEXT_ID},
};

const BOLD: &str = " penwidth=3";

pub fn render(lattice: &Lattice, dict: &IPADic) -> Result<String, Box<dyn Error>> {
    let cursor = (lattice.dp.len() - 1, 0);
    let len = lattice.indices.len();
    let best_path = lattice.find_best_path();
    let mut dot = String::from("");
    writeln!(
        dot,
        r#"digraph lattice {{
  rankdir=LR;
  splines=polyline;
  nodesep=.05;
  
  BOS [label="BOS\n0 (0)" shape="doublecircle"{}];
  EOS [label="EOS\n{} (0)" shape="doublecircle"{}];
"#,
        BOLD,
        lattice.dp[cursor.0].get(cursor.1).unwrap().0,
        BOLD
    )?;
    for (i, index) in lattice.indices.iter().enumerate() {
        for (j, (left_wid, wlen)) in index.iter().enumerate() {
            let left = dict.get_word(left_wid).unwrap();
            let node_style = match &best_path {
                Some(best_path) if best_path.contains(&(i + 1, j)) => BOLD,
                _ => "",
            };
            writeln!(
                dot,
                r#"  "{}_{}" [label="{}\n({}, {})"{}];"#,
                i,
                j,
                left_wid.get_surface(),
                lattice.dp[i + 1][j].0,
                left.cost,
                node_style,
            )?;
            if i == 0 {
                let right = left;
                let cost = dict
                    .transition_cost(BOS_CONTEXT_ID, right.right_context_id)
                    .unwrap();
                let bos_edge_style = match &best_path {
                    Some(best_path) if best_path.contains(&(i + 1, j)) => BOLD,
                    _ => "",
                };
                writeln!(
                    dot,
                    r#"  BOS -> "{}_{}" [label="({})"{}];"#,
                    i, j, cost, bos_edge_style
                )?;
            }
            if i + wlen >= len {
                let cost = dict
                    .transition_cost(left.left_context_id, EOS_CONTEXT_ID)
                    .unwrap();
                let eos_edge_style = match &best_path {
                    Some(best_path) if best_path.contains(&(i + 1, j)) => BOLD,
                    _ => "",
                };
                writeln!(
                    dot,
                    r#"  "{}_{}" -> EOS [label="({})"{}];"#,
                    i, j, cost, eos_edge_style
                )?;
                continue;
            }
            for (k, (right_wid, _)) in lattice.indices[i + wlen].iter().enumerate() {
                let right = dict.get_word(right_wid).unwrap();
                let cost = dict
                    .transition_cost(left.left_context_id, right.right_context_id)
                    .unwrap();
                let edge_style = match &best_path {
                    Some(best_path)
                        if best_path.contains(&(i + 1, j))
                            && best_path.contains(&(i + wlen + 1, k)) =>
                    {
                        BOLD
                    }
                    _ => "",
                };
                writeln!(
                    dot,
                    r#"  "{}_{}" -> "{}_{}" [label="({})"{}];"#,
                    i,
                    j,
                    i + wlen,
                    k,
                    cost,
                    edge_style
                )?;
            }
        }
    }
    writeln!(dot, "}}")?;
    Ok(dot)
}
