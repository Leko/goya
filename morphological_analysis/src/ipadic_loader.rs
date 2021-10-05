use super::char_class::{CharClass, CharClassifier, CharDefinition, InvokeTiming};
use super::ipadic::IPADic;
use super::morpheme::Morpheme;
use encoding_rs::EUC_JP;
use glob::glob;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::Path;
use std::vec::Vec;
use std::{fs, vec};

const COL_SURFACE_FORM: usize = 0; // 表層形
const COL_LEFT_CONTEXT_ID: usize = 1; // 左文脈ID
const COL_RIGHT_CONTEXT_ID: usize = 2; // 右文脈ID
const COL_COST: usize = 3; // コスト

pub fn load(dir: &str) -> Result<IPADic, Box<dyn Error>> {
    let classes = load_chars(Path::new(dir).join("char.def"))?;
    let matrix = load_matrix(Path::new(dir).join("matrix.def"))?;
    let unknown = load_unknown(Path::new(dir).join("unk.def"))?;
    let csv_pattern = Path::new(dir).join("*.csv");
    let csv_pattern = csv_pattern.to_str().ok_or("Failed to build glob pattern")?;

    let mut vocabulary = HashMap::new();
    let mut homonyms = HashMap::new();
    let mut id = 1;
    for path in glob(csv_pattern)? {
        for word in load_words_csv(path?)? {
            homonyms
                .entry(word.surface_form.to_string())
                .or_insert_with(Vec::new)
                .push(id);
            vocabulary.insert(id, word);
            id += 1;
        }
    }

    let mut unknown_vocabulary = HashMap::new();
    let mut unknown_classes = HashMap::new();
    let mut id = 1;
    for (class, words) in unknown.into_iter() {
        for word in words {
            unknown_vocabulary.insert(id, word);
            unknown_classes
                .entry(class.to_string())
                .or_insert_with(Vec::new)
                .push(id);
            id += 1;
        }
    }
    Ok(IPADic::from(
        vocabulary,
        homonyms,
        classes,
        matrix,
        unknown_classes,
        unknown_vocabulary,
    ))
}

fn load_words_csv<P>(path: P) -> Result<Vec<Morpheme>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let mut rdr = csv::Reader::from_reader(utf8.as_bytes());
    let mut words = vec![];
    for row in rdr.records() {
        let row = row?;
        words.push(Morpheme::new(
            row[COL_SURFACE_FORM].to_string(),
            row[COL_LEFT_CONTEXT_ID].parse::<usize>().unwrap(),
            row[COL_RIGHT_CONTEXT_ID].parse::<usize>().unwrap(),
            row[COL_COST].parse::<i16>().unwrap(),
            row.iter()
                .skip(COL_COST + 1)
                .map(|v| v.to_string())
                .collect::<Vec<_>>(),
        ))
    }
    Ok(words)
}

fn load_chars<P>(path: P) -> Result<CharClassifier, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let lines = utf8
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| Regex::new(r"#.*$").unwrap().replace(line, ""))
        .collect::<Vec<_>>();

    let head = lines.iter().take_while(|line| {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        !parts[0].starts_with("0x")
    });
    let mut chars = HashMap::new();
    for line in head {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let kind = parts[0].to_owned();
        let class = kind.to_string();
        let timing = if parts[1] == "0" {
            InvokeTiming::Fallback
        } else {
            InvokeTiming::Always
        };
        let group_by_same_kind = parts[2] == "1";
        let len = parts[3].parse::<usize>()?;
        chars.insert(
            kind,
            CharDefinition {
                class,
                timing,
                group_by_same_kind,
                len,
                compatibilities: HashSet::new(),
            },
        );
    }

    let tail = lines.iter().skip_while(|line| {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        !parts[0].starts_with("0x")
    });
    let mut ranges = vec![];
    for line in tail {
        let parts = line.trim().split_ascii_whitespace().collect::<Vec<_>>();
        let range = parts[0]
            .split("..")
            .map(|c| u32::from_str_radix(&c[2..], 16).unwrap())
            .map(|c| char::from_u32(c).unwrap())
            .collect::<Vec<_>>();
        let range = if range.len() > 1 {
            (range[0] as u32, range[1] as u32)
        } else {
            (range[0] as u32, range[0] as u32)
        };
        let class = parts[1];
        let compatibilities = parts
            .iter()
            .skip(2)
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        chars.get_mut(class).unwrap().compatibilities = compatibilities;
        ranges.push(CharClass::from(range, class.to_string()));
    }

    Ok(CharClassifier::from(chars, ranges))
}

fn load_matrix<P>(path: P) -> Result<Vec<Vec<i16>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let eucjp = fs::read(path)?;
    let (utf8, _, _) = EUC_JP.decode(&eucjp);
    let mut lines = utf8.lines();
    let size = lines
        .next()
        .expect("failed to read the first line")
        .split_ascii_whitespace()
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut matrix = vec![vec![-1; size[1]]; size[0]];
    for line in lines {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        let left = parts[0].parse::<usize>()?;
        let right = parts[1].parse::<usize>()?;
        let cost = parts[2].parse::<i16>()?;
        matrix[left][right] = cost;
    }
    Ok(matrix)
}

fn load_unknown<P>(path: P) -> Result<HashMap<String, Vec<Morpheme>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let words = load_words_csv(path)?;
    let mut map = HashMap::<String, Vec<Morpheme>>::new();
    for w in words.into_iter() {
        map.entry(w.surface_form.to_string())
            .or_insert_with(Vec::new)
            .push(w);
    }
    Ok(map)
}
