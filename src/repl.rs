//! Starts a REPL for Lox
use crate::lox::Lox;
use std::{io, io::Write};
#[allow(unreachable_code)]
pub(crate) fn start_repl() -> std::io::Result<()> {
    let mut lox_interpreter = Lox::init_interpreter();
    let mut buf = String::new();
    loop {
        print_prompt(&mut buf)?;
        let input = buf.trim();
        if input == "exit" || input == "quit" {
            println!("Exiting Lox interpreter");
            std::process::exit(0);
        }
        lox_interpreter.interpret(input);
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
