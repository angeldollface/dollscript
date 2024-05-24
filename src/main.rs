/*
THE DOLLSCRIPT INTERPRETER
by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the CLI
/// function.
use dollscript::cli;

/// The main point
/// of entry for the
/// Rust compiler.
fn main() -> () {
    match cli(){
        Ok(_x) => {},
        Err(e) => {
            println!("{}", &e.to_string());
        }
    };
}
