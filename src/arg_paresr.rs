
use std::env;

pub fn arg_parser() -> InputType{
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => InputType::Repl,
        Some(param) => match param.as_str() {
            "--help"=> InputType::Help,
            "--version"=> InputType::Version,
            file_name => {
                InputType::ExecuteFile(file_name.to_string())
            }   
        },
    }
}

pub enum InputType{
    ExecuteFile(String),
    Help,
    Repl,
    Version
}
