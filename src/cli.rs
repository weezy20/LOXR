use crate::lox::Lox;
use crate::repl;
use std::path::Path;

/// Start a REPL for Lox if no CLI args are passed
/// Or, accept a file path, parse it and try running it as a Lox file
pub(crate) fn run_cli() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let file_path = Path::new(&args[1]);
        let file = std::fs::read_to_string(file_path)
            .expect("Cannot access file path {file_path:?}");
        run_file(file.as_ref());
    } else if repl::start_repl().is_err() {
        panic!("REPL error");
    } else if args.len() > 2 {
        eprintln!("Usage \"rlox --{{lox file}}\"");
    }
}

/// Run an entire Lox file
pub(in crate::cli) fn run_file(file: &str) {
    let mut lox = Lox::new();
    lox.run_file(String::from(file));
}
