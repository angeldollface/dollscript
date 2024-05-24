/*
THE DOLLSCRIPT INTERPRETER 
by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// Importing the 
/// "App" structure from
/// the "cliply" crate to
/// build a CLI app.
use cliply::App;

/// Importing the error-handling structure
/// so errors can be hanlded.
use super::err::DollscriptErr;

/// Dollscript's CLI.
pub fn cli() -> Result<(), DollscriptErr> {
    let mut dolly: App = App::new(
        &"The Dollscript Interpreter",
        &"0.1.0",
        &"Angel Dollface"
    );
    dolly.add_arg(
        &"do",
        &"execute a Dollscript file",
        &"true"
    );
    dolly.add_arg(
        &"glam",
        &"lints a Dollscript file",
        &"true"
    );
    dolly.add_arg(
        &"speak",
        &"starts the interactive shell",
        &"true"
    );
    if dolly.version_is(){
        println!("{}", dolly.version_info());
    }
    else if dolly.help_is(){
        println!("{}", dolly.help_info());
    }
    else if dolly.arg_was_used(&"do"){
        let arg_data: String = match dolly.get_arg_data(&"do"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<(), DollscriptErr>(DollscriptErr::new(&e.to_string()))
        };
        // Process further.
    }
    else if dolly.arg_was_used(&"glam"){
        let arg_data: String = match dolly.get_arg_data(&"glam"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<(), DollscriptErr>(DollscriptErr::new(&e.to_string()))
        };
        // Process further
    }
    else if dolly.arg_was_used(&"speak"){
        let arg_data: String = match dolly.get_arg_data(&"speak"){
            Ok(arg_data) => arg_data,
            Err(e) => return Err::<(), DollscriptErr>(DollscriptErr::new(&e.to_string()))
        };
        // Process further.
    }
    else {
        println!("{}", dolly.help_info());
    }
    Ok(())
}
