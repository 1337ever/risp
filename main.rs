extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".risp-history").is_err() {
        eprintln!("No history found.");
    }

    loop {
        let readline = rl.readline("()> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                rl.save_history(".risp-history").unwrap();
                if line.len() > 0 {
                    println!("{}", line);
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}