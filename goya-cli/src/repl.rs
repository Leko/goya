use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::{IPADic, WordIdentifier};
use morphological_analysis::lattice::Lattice;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn start(da: &DoubleArray, dict: &IPADic) {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                let lattice = Lattice::parse(&line, da, dict);
                // println!("{}", lattice.as_dot(dict));

                if let Some(path) = lattice.find_best() {
                    for wid in path.into_iter() {
                        let word = dict.get_word(&wid).unwrap();
                        if let WordIdentifier::Unknown(_, surface_form) = wid {
                            // TODO: Display actual matched unknown text
                            println!("{}\t{}", surface_form, word.meta.join(","));
                        } else {
                            println!("{}\t{}", word.surface_form, word.meta.join(","));
                        }
                    }
                    println!("EOS");
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
