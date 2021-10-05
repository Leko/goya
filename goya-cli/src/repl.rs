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
                // writeln!(out,"{}", lattice.as_dot(dict))?;
                if let Some(path) = lattice.find_best() {
                    for wid in path.into_iter() {
                        let word = dict.get_word(&wid).unwrap();
                        if let WordIdentifier::Unknown(_, surface_form) = wid {
                            writeln!(out, "{}\t{}", surface_form, word.features.join(","))?;
                        } else {
                            writeln!(out, "{}\t{}", word.surface_form, word.features.join(","))?;
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
