/// Debug feature must be enabled to use this
/// Log the source file and line number where the error occured
#[macro_export]
macro_rules! loc {
    // TODO: optional msg parameter to print a custom msg
    ($($msg:expr)*) => {
        #[cfg(feature = "debug")]
        {
            $(eprintln!("{}{}", "DEBUG#> ".bright_red(), $msg);)*
            eprintln!("(File : {} Line : {})", file!().bright_yellow(), (line!() + 1).to_string().green());
        }
    };
}
