
use crate::common::executable::ExecutableLine;
use super::runtime::Runtime;
use crate::common::errors::Result;
mod exit;
mod println;
mod assign;


pub fn execute(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    // this is not good idea
    // try to store closure in ExecutableLine
    match executable.function_name.as_str() {
        "println" => println::println(runtime, executable),
        "assign" => assign::assign(runtime, executable),
        "exit" => exit::exit(runtime, executable),
        _ => todo!()
    }
}