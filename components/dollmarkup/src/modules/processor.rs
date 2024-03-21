/*
DOLLMARKUP by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the GNU GPL v.3.0.
*/

/// Importing Mandy's error
/// structure to catch and return
/// errors.
use super::err::DMUErr;

/// Importing the "Token"
/// data structure to
/// cast everything strictly.
use super::lexer::Token;

/// Importing the method
/// to tokenize input
/// string.
use super::lexer::tokenize;

/// Importing the method
/// to parse a token sequence
/// into a set of statements.
use super::ast::parse_tokens;

pub struct Deserializer<'de> {
    // This string starts with the input data and characters are truncated off
    // the beginning as data is parsed.
    input: &'de str,
}

impl<'de> Deserializer<'de> {
    // By convention, `Deserializer` constructors are named like `from_xyz`.
    // That way basic use cases are satisfied by something like
    // `serde_json::from_str(...)` while advanced use cases that require a
    // deserializer can make one with `serde_json::Deserializer::from_str(...)`.
    pub fn from_str(input: &'de str) -> Self {
        Deserializer { input }
    }
}