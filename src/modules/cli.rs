use cleasy::App;

pub fn cli() {
    let mut doll_script_cli: App = App::new(
        &"Dollscript",
        &"1.0.0",
        &"Angel Dollface"
    );
    doll_script_cli.add_arg(
        &"inf",
        &" script input file",
        &"true"
    );
    doll_script_cli.add_arg(
        &"ouf", 
        &" output file", 
        &"true"
    );
    if doll_script_cli.version_is() {
        println!(
            "{}", 
            doll_script_cli.version().to_string()
        );
    }
    else if doll_script_cli.help_is() {
        println!(
            "{}", 
            doll_script_cli.help().to_string()
        );
    }
    else if doll_script_cli.arg_was_used(&"shl") {
        // Launch shell.

    }
    else if doll_script_cli.arg_was_used(&"inf") {
        let input_file: String = doll_script_cli.get_arg_data(&"inf");
        // Process script file.

    }
    else if doll_script_cli.arg_was_used(&"inf") &&
        doll_script_cli.arg_was_used(&"out") {
            let input_file: String = doll_script_cli.get_arg_data(&"inf");
            let input_file: String = doll_script_cli.get_arg_data(&"out");
            // Compile script file into the desired output format.
            
    }
    else {
        println!(
            "{}", 
            doll_script_cli.help()
        );
    }
}
