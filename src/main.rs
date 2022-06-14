// #![feature(iter_advance_by)]
#![feature(let_chains)]
mod _lox_;
mod cli;
mod tests;

fn main() {
    cli::run_cli();
}
