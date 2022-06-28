use _lox_::Lox;
use std::fs::File;
use std::io::Read;
/// Start a REPL for Lox if no CLI args are passed
/// Or, accept a file path, parse it and try running it as a Lox file
pub fn run_cli() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let mut file_path = File::open(&args[1]).expect(&format!("Cannot open file {}", &args[1]));
        let mut file = String::new();
        file_path
            .read_to_string(&mut file)
            .expect("Cannot access file path {file_path}");
        run_file(file.as_ref());
    } else if repl::start_repl().is_err() {
        panic!("REPL error");
    } else if args.len() > 2 {
        eprintln!("Usage \"rlox {{lox file}}\"");
    }
}
pub(in crate::cli) fn run_file(file: &str) {
    let mut lox = Lox::new(file.into());
    lox.run(file.into());
}

mod repl {
    use super::*;

    use std::{io, io::Write};
    #[allow(unreachable_code)]
    pub(crate) fn start_repl() -> std::io::Result<()> {
        let mut lox_interpreter = Lox::new(Default::default());
        let mut buf = String::new();
        loop {
            print_prompt(&mut buf)?;
            let input = buf.trim();
            if input == "exit" || input == "quit" {
                println!("Exiting Lox interpreter");
                std::process::exit(0);
            }
            lox_interpreter.run(String::from(input));
            buf.clear();
        }
        Ok(())
    }

    #[inline(always)]
    fn print_prompt(buf: &mut String) -> io::Result<()> {
        // Edit this print argument to use your app's name
        print!("Lox > ");
        io::stdout().lock().flush()?;
        io::stdin().read_line(buf)?;
        Ok(())
    }
}
