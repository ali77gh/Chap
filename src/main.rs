mod builtin_function;
mod common;
mod compile;
mod runners;
mod runtime;
#[path = "2c/mod.rs"]
mod to_c;

use common::{errors::Result, help::show_help, version::show_version};
use runners::file_executor::file_executor;
use runners::repl::start_repl;

fn main() -> Result<()> {
    match arg_parser() {
        InputType::ExecuteFile(file_name) => {
            file_executor(&file_name);
        }
        InputType::CompileToC(input, output) => match input {
            Some(input) => compile_to_c_runner(&input, output.as_deref()),
            None => {
                println!("usage: chap --to-c <input.chap> [output.c]");
                show_help();
            }
        },
        InputType::Help => show_help(),
        InputType::Version => show_version(),
        InputType::Repl => start_repl(),
    }

    Ok(())
}

// compiles a chap script to C code (chap --to-c input.chap [output.c])
fn compile_to_c_runner(input: &str, output: Option<&str>) {
    let code = read_to_string(input).unwrap();
    let c_code = match to_c::compile_to_c(&code) {
        Ok(c_code) => c_code,
        Err(e) => {
            e.exit_with_error();
            return;
        }
    };
    match output {
        Some(file_name) => std::fs::write(file_name, c_code).unwrap(),
        None => println!("{}", c_code),
    }
}

use std::env;
use std::fs::read_to_string;

pub fn arg_parser() -> InputType {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        None => InputType::Repl,
        Some(param) => match param.as_str() {
            "--help" | "-h" => InputType::Help,
            "--version" | "-v" => InputType::Version,
            "--to-c" | "-c" => {
                InputType::CompileToC(args.get(2).map(|x| x.to_string()), args.get(3).cloned())
            }
            file_name => InputType::ExecuteFile(file_name.to_string()),
        },
    }
}

pub enum InputType {
    ExecuteFile(String),
    CompileToC(Option<String>, Option<String>),
    Help,
    Repl,
    Version,
}
