
/// Debug feature must be enabled to use this
/// Log the source file and line number where the error occured
#[macro_export]
macro_rules! loc {
    // https://github.com/rust-lang/rfcs/blob/master/text/2298-macro-at-most-once-rep.md
    ($($msg:expr)?) => {
        #[cfg(feature = "debug")]
        {
            use colored::Colorize;
            eprintln!("(File : {} Line : {})", file!().bright_yellow(), (line!() + 1).to_string().green());
            $(eprintln!("{}\"{}\"", "DEBUG#> ".bright_red(), $msg);)?
        }
    };
}
