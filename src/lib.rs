/*
THE DOLLSCRIPT INTERPRETER 
by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Declaring the "modules" directory
/// as a module.
pub mod modules;

/// Re-exporting the
/// error-handling module.
pub use modules::err::*;

/// Re-exporting the CLI
/// module.
pub use modules::cli::*;

/// Re-exporting the module
/// for lexing code.
pub use modules::lexer::*;
