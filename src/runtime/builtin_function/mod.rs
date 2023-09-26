
use crate::common::{executable::ExecutableLine, errors::ChapError};
use self::math::{add::add, equal::equal, not_equal::not_equal};

use super::runtime::Runtime;
use crate::common::errors::Result;

mod exit;
mod println;
mod input;
mod assign;

mod control_flow;
mod math;

pub fn execute(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    // this is not good idea
    // try to store closure in ExecutableLine
    match executable.function_name.as_str() {
        "jump" => control_flow::jump::jump(runtime, executable),
        "jump_if" => control_flow::jump_if::jump_if(runtime, executable),
        "assign" => assign::assign(runtime, executable),
        "add" => add(runtime, executable),
        "equal" => equal(runtime, executable),
        "not_equal" => not_equal(runtime, executable),

        "println" => println::println(runtime, executable),
        "new_tag" => control_flow::new_tag::new_tag(runtime, executable),
        "input" => input::input(runtime, executable),
        "exit" => exit::exit(runtime, executable),
        _ => Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no '{}' builtin function",executable.function_name)
            ))
    }
}
