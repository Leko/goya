use morphological_analysis::double_array::DoubleArray;
use morphological_analysis::extractor::extract;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::collections::HashMap;

fn demodict() -> DoubleArray {
    // registered words: "a" and "bc"
    let mut codes: HashMap<char, usize> = HashMap::new();
    codes.insert('\0', 0);
    codes.insert('a', 1);
    codes.insert('b', 2);
    codes.insert('c', 3);
    let base: Vec<i32> = vec![0, 3, 0, -1, 3, 3, 7, -1];
    let check: Vec<usize> = vec![0, 0, 0, 4, 1, 1, 5, 6];
    return DoubleArray::new(base, check, codes);
}

pub fn start() {
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {}
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let result = extract(&line, &demodict());
                println!("{:?}", result);
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
