use _lox_::Lox;
use std::fs::File;
use std::io::Read;
/// Start a REPL for Lox if no CLI args are passed
/// Or, accept a file path, parse it and try running it as a Lox file
pub fn run_cli() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        // TODO: This is unreliable
        let mut file_path = File::open(&args[1]).expect(&format!("Cannot open file {}", &args[1]));
        let mut file = String::new();
        file_path
            .read_to_string(&mut file)
            .expect("Cannot access file path {file_path}");
        run_file(file.as_ref());
    } else if repl::start_repl().is_err() {
        panic!("REPL error");
    } else if args.len() > 2 {
        eprintln!("Usage \"loxr {{lox file}}\"");
    }
}
pub fn run_file(file: &str) {
    let mut lox = Lox::new(file.into());
    lox.run(None);
    if lox.had_runtime_error {
        std::process::exit(70);
    }
}

mod repl {
    use super::*;
    use rustyline::{error::ReadlineError, Editor};
    // use rustyline::validate::MatchingBracketValidator;
    // use rustyline::{Cmd, EventHandler, KeyCode, KeyEvent, Modifiers};
    // use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

    // #[derive(Completer, Helper, Highlighter, Hinter, Validator)]
    // struct InputValidator {
    //     #[rustyline(Validator)]
    //     brackets: MatchingBracketValidator,
    // }

    #[allow(unreachable_code)]
    pub(crate) fn start_repl() -> std::io::Result<()> {
        let mut lox_interpreter = Lox::new(Default::default());
        #[allow(unused_assignments)]
        let mut buf = String::new();
        // let h = InputValidator {
        //     brackets: MatchingBracketValidator::new(),
        // };
        let mut rl = Editor::<()>::new().expect("rustyline failed");
        // rl.set_helper(Some(h));
        // rl.bind_sequence(
        //     KeyEvent(KeyCode::Char('s'), Modifiers::CTRL),
        //     EventHandler::Simple(Cmd::Newline),
        // );
        if rl.load_history("history.txt").is_err() {
            // println!("No previous history.");
        }
        loop {
            let line = rl.readline("Lox > ");
            match line {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    buf = line;
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    println!("Exiting Lox interpreter");
                    std::process::exit(0);
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(e) => {
                    eprintln!("Unexpected prompt error : {e:?}");
                    std::process::exit(1);
                }
            }
            let input: &str = buf.trim();
            if input == "exit" || input == "quit" {
                println!("Exiting Lox interpreter");
                std::process::exit(0);
            }
            if input.starts_with("//") || input.starts_with("/*") && input.ends_with("*/") {
                continue;
            }
            if let Some(semicolon) = input.chars().last() {
                if semicolon != ';' && semicolon != '}' {
                    let mut s = input.to_string();
                    s.push(';');
                    lox_interpreter.run(Some(s));
                    continue;
                }
            }
            lox_interpreter.run(Some(String::from(input)));
            buf.clear();
        }
        Ok(())
    }
}
