use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::extractor::extract;
use morphological_analysis::ipadic::IPADic;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn start(da: DoubleArray, dict: &IPADic) {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {}
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                let result = extract(&line, &da);
                if result.tokens.is_empty() {
                    println!("not found");
                }
                for t in result.tokens {
                    if let Some(id) = t.id {
                        if let Some(word) = dict.get(&id) {
                            println!("{:#?}", word);
                        }
                    }
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
    rl.save_history("history.txt").unwrap();
}
