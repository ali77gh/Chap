mod builtin_function;
mod common;
mod compile;
mod runners;
mod runtime;
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
        InputType::Compile(input, output) => match input {
            Some(input) => compile_runner(&input, output.as_deref()),
            None => {
                println!("usage: chap compile <input.chap> [output]");
                show_help();
            }
        },
        InputType::Help => show_help(),
        InputType::Version => show_version(),
        InputType::Repl => start_repl(),
    }

    Ok(())
}

// compiles chap source code into C code, exits process on chap errors
fn generate_c_or_exit(input: &str) -> String {
    let code = read_to_string(input).unwrap();
    match to_c::compile_to_c(&code) {
        Ok(c_code) => c_code,
        Err(e) => {
            e.exit_with_error();
            unreachable!()
        }
    }
}

// compiles a chap script to C code (chap --to-c input.chap [output.c])
fn compile_to_c_runner(input: &str, output: Option<&str>) {
    let c_code = generate_c_or_exit(input);

    match output {
        Some(file_name) => std::fs::write(file_name, c_code).unwrap(),
        None => println!("{}", c_code),
    }
}

// compiles a chap script to a native executable with gcc
// (chap compile input.chap [output])
fn compile_runner(input: &str, output: Option<&str>) {
    use std::process::Command;

    let c_code = generate_c_or_exit(input);

    // input.chap -> input
    let base = output
        .map(|x| x.to_string())
        .unwrap_or_else(|| input.trim_end_matches(".chap").to_string());
    let c_file_name = format!("{}.c", base);
    std::fs::write(&c_file_name, c_code).unwrap();

    let status = Command::new("gcc")
        .arg("-O2")
        .arg(&c_file_name)
        .arg("-o")
        .arg(&base)
        .arg("-lm") // generated code uses math lib (sqrt)
        .status()
        .expect("failed to run gcc (is it installed?)");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
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
            "compile" => {
                InputType::Compile(args.get(2).map(|x| x.to_string()), args.get(3).cloned())
            }
            file_name => InputType::ExecuteFile(file_name.to_string()),
        },
    }
}

pub enum InputType {
    ExecuteFile(String),
    CompileToC(Option<String>, Option<String>),
    Compile(Option<String>, Option<String>),
    Help,
    Repl,
    Version,
}
