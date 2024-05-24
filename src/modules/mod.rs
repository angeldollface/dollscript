/*
THE DOLLSCRIPT INTERPRETER
by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Exporting the
/// error-handling module.
pub mod err;

/// Exporting the CLI
/// module.
pub mod cli;

/// Exporting the module
/// for testing things.
#[cfg(test)]
pub mod tests;

/// Exporting the lexing
/// module.
pub mod lexer;
