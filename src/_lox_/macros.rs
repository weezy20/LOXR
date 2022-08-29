/// Log the source file and line number where the error occured
#[macro_export]
macro_rules! loc {
    () => {
        #[cfg(feature = "debug")]
        eprintln!("(File : {} Line : {})", file!().bright_yellow(), (line!() + 1).to_string().green());
    };
}
