use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::ipadic::IPADic;
use morphological_analysis::lattice::Lattice;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn start(da: DoubleArray, dict: &IPADic) {
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
                let lattice = Lattice::parse(&line, &da);
                // println!("{}", lattice.as_dot(dict));

                let words = lattice
                    .find_best(dict)
                    .iter()
                    .map(|wid| dict.get(wid).unwrap())
                    .collect::<Vec<_>>();
                for w in words {
                    println!(
                        "{}\t{:?},{},{},{},{},{},{},{},{}",
                        w.surface_form,
                        w.lexical_category,
                        w.lexical_subcategory1
                            .as_ref()
                            .unwrap_or(&String::from("*")),
                        w.lexical_subcategory2
                            .as_ref()
                            .unwrap_or(&String::from("*")),
                        w.lexical_subcategory3
                            .as_ref()
                            .unwrap_or(&String::from("*")),
                        "*",
                        w.conjugation.as_ref().unwrap_or(&String::from("*")),
                        w.infinitive,
                        w.ruby,
                        w.pronounciation,
                    );
                }
                println!("EOS");
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
