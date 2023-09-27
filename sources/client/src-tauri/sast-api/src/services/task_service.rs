use run_script::ScriptOptions;

pub fn execute_command(command: String) -> String {
    let options = ScriptOptions::new();
    let args = vec![];

    let denoted_command = format!(r#"{}"#, command);

    let (_, output, _) = run_script::run(&denoted_command, &args, &options).unwrap();

    return output;
}
