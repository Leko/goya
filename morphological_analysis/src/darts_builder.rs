use super::common_prefix_tree::CommonPrefixTree;
use super::darts::Darts;
use super::vocabulary::Word;
use indexmap::IndexSet;
use std::collections::HashMap;

const DEFAULT_SIZE: usize = 1024 * 1024 * 5;
const ROOT_ID: usize = 1;
const TERM_CHAR: char = '\0';
const TERM_CODE: usize = 0;

#[derive(Debug)]
pub struct DartsBuilder {
    size: usize,
}

impl DartsBuilder {
    pub fn new() -> DartsBuilder {
        DartsBuilder { size: DEFAULT_SIZE }
    }

    pub fn build(&self, words: &HashMap<usize, Word>) -> Darts {
        let mut base: Vec<i32> = vec![0; self.size];
        let mut check = vec![0; self.size];
        let mut codes = IndexSet::new();
        let mut cpt = CommonPrefixTree::new();

        base[ROOT_ID] = 1;
        check[ROOT_ID] = 0;
        assert_eq!(codes.insert_full(TERM_CHAR).0, TERM_CODE);

        for (id, word) in words.iter() {
            cpt.append(*id, &word.surface_form);
        }

        for (prefix, tree) in cpt.entires_dfs().iter().take(100) {
            println!("{:?} -> {:?}", prefix, tree.can_stop());
        }

        // println!(
        //     "{:?}",
        //     cpt.entires_dfs()
        //         .iter()
        //         .map(|(prefix, _)| prefix)
        //         .take(100)
        //         .collect::<Vec<_>>()
        // );

        Darts { base, check, codes }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::super::ipadic::IPADic;
//     use super::*;
//     use std::env;
//     use std::path::PathBuf;

//     #[test]
//     fn build_ipadic() {
//         let dic_dir = PathBuf::from(env::current_dir().unwrap())
//             .join("..")
//             .join("mecab")
//             .join("mecab-ipadic");
//         let ipadic = IPADic::from_dir(&dic_dir.to_str().unwrap().to_string()).unwrap();
//         let builder = DartsBuilder::new();
//         builder.build(&ipadic.vocabulary);
//         panic!("hoge");
//     }
// }
