use morphological_analysis::dot;
use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::{IPADic, WordIdentifier};
use morphological_analysis::lattice::Lattice;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

pub fn start(da: &DoubleArray, dict: &IPADic) -> Result<(), std::io::Error> {
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
                        let word = dict.get_word(&wid).unwrap();
                        match wid {
                            WordIdentifier::Unknown(_, surface_form) => {
                                writeln!(out, "{}\t{}", surface_form, word.features.join(","))?;
                            }
                            WordIdentifier::Known(_, surface_form) => {
                                writeln!(out, "{}\t{}", surface_form, word.features.join(","))?;
                            }
                        }
                    }
                    out.write(b"EOS\n")?;
                    out.flush()?;
                }
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}
