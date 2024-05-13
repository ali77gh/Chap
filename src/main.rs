mod builtin_function;
mod common;
mod compile;
mod runners;
mod runtime;

use common::{errors::Result, help::show_help, version::show_version};
use runners::file_executor::file_executor;
use runners::repl::start_repl;

fn main() -> Result<()> {
    match arg_parser() {
        InputType::ExecuteFile(file_name) => {
            file_executor(&file_name)?;
        }
        InputType::Help => show_help(),
        InputType::Version => show_version(),
        InputType::Repl => start_repl(),
    }

    Ok(())
}

use std::env;

pub fn arg_parser() -> InputType {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => InputType::Repl,
        Some(param) => match param.as_str() {
            "--help" | "-h" => InputType::Help,
            "--version" | "-v" => InputType::Version,
            file_name => InputType::ExecuteFile(file_name.to_string()),
        },
    }
}

pub enum InputType {
    ExecuteFile(String),
    Help,
    Repl,
    Version,
}
