
use crate::common::{executable::ExecutableLine, errors::ChapError};
use super::runtime::Runtime;
use crate::common::errors::Result;

mod exit;
mod println;
mod input;
mod assign;


pub fn execute(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    // this is not good idea
    // try to store closure in ExecutableLine
    match executable.function_name.as_str() {
        "println" => println::println(runtime, executable),
        "input" => input::input(runtime, executable),
        "assign" => assign::assign(runtime, executable),
        "exit" => exit::exit(runtime, executable),
        _ => Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no '{}' builtin function",executable.function_name)
            ))
    }
}