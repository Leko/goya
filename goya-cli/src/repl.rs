use goya::dot;
use goya::double_array::DoubleArray;
use goya::id::WordIdentifier;
use goya::ipadic::IPADic;
use goya::lattice::Lattice;
use goya::word_set::WordSet;
use std::io::{stdin, stdout, BufRead, BufWriter, Write};

pub struct ReplOption<'a> {
    pub da: &'a DoubleArray,
    pub dict: &'a IPADic,
    pub word_set: &'a WordSet,
}

pub fn start(opt: ReplOption) -> Result<(), std::io::Error> {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    for line in stdin().lock().lines() {
        match line {
            Ok(line) if line.is_empty() => continue,
            Ok(line) => {
                let lattice = Lattice::parse(&line, opt.da, opt.dict);
                // writeln!(out, "{}", dot::render(&lattice, opt.dict).unwrap())?;
                if let Some(path) = lattice.find_best() {
                    for wid in path.into_iter() {
                        match wid {
                            WordIdentifier::Unknown(id, surface_form) => {
                                let features = &opt.word_set.get_unknown(&id).unwrap().features;
                                writeln!(out, "{}\t{}", surface_form, features.join(","))?;
                            }
                            WordIdentifier::Known(id, surface_form) => {
                                let features = &opt.word_set.get_known(&id).unwrap().features;
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
