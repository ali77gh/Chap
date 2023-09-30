
use crate::common::{executable::{BuiltinFunction, ExecutableLine}, errors::ChapError};

use crate::common::errors::Result;

mod exit;
mod println;
mod input;
mod assign;

mod control_flow;
mod math;
mod utils;

pub fn closure_gen(executable:&ExecutableLine) -> Result<BuiltinFunction>{

    //TODO normilize functions names

    Ok(match executable.function_name.as_str() {
        "jump" => control_flow::jump::jump,
        "jump_if" => control_flow::jump_if::jump_if,
        "jump_if_equal" => control_flow::jump_if_equal::jump_if_equal,
        "jump_if_not_equal" => control_flow::jump_if_not_equal::jump_if_not_equal,
        "assign" => assign::assign,
        "add" => math::add::add,
        "increase" => math::increase::increase,
        "decrease" => math::decrease::decrease,
        "equal" => math::equal::equal,
        "not_equal" => math::not_equal::not_equal,
        "println" => println::println,
        "new_tag" => control_flow::new_tag::new_tag,
        "input" => input::input,
        "exit" => exit::exit,
        _ => return Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no '{}' builtin function",executable.function_name)
            ))
    })
}


