use morphological_analysis::dot;
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::{IPADic, WordIdentifier};
use morphological_analysis::lattice::Lattice;
use morphological_analysis::word_set::WordSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

pub fn start(da: &DoubleArray, dict: &IPADic, word_set: &WordSet) -> Result<(), std::io::Error> {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for line in stdin().lock().lines() {
        match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => {
                let lattice = Lattice::parse(&line, da, dict);
                writeln!(out, "{}", dot::render(&lattice, dict).unwrap())?;
                if let Some(path) = lattice.find_best() {
                    for wid in path.into_iter() {
                        match wid {
                            WordIdentifier::Unknown(id, surface_form) => {
                                let features = &word_set.unknown.get(&id).unwrap().features;
                                writeln!(out, "{}\t{}", surface_form, features.join(","))?;
                            }
                            WordIdentifier::Known(id, surface_form) => {
                                let features = &word_set.known.get(&id).unwrap().features;
                                writeln!(out, "{}\t{}", surface_form, features.join(","))?;
                            }
                        }
                    }
                    writeln!(out, "EOS")?;
                    out.flush()?;
                }
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
