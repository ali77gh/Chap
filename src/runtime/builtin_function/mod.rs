
use crate::common::{executable::{BuiltinFunction, ExecutableLine}, errors::ChapError};

use crate::common::errors::Result;

mod exit;
mod println;
mod input;
mod assign;

mod control_flow;
mod math;
mod utils;

pub fn closure_gen(executable: &ExecutableLine) -> Result<BuiltinFunction>{

    let function_name = executable.function_name
        .clone()
        .to_lowercase()
        .replace(" ", "")
        .replace("_", "");

    Ok(match function_name.as_str() {
        "assign" => assign::assign,
        "jump" => control_flow::jump::jump,
        "jumpif" => control_flow::jump_if::jump_if,
        "jumpifnot" => control_flow::jump_if_not::jump_if_not,
        "jumpifequal" | "jumpeq" => control_flow::jump_if_equal::jump_if_equal,
        "jumpifnotequal" | "jumpneq"=> control_flow::jump_if_not_equal::jump_if_not_equal,
        "newtag" => control_flow::new_tag::new_tag,

        "add" => math::add::add,
        "addmany" | "addall" => math::add_many::add_many,
        "minus" => math::minus::minus,
        "multiply" => math::multiply::multiply,
        "divide" => math::divide::divide,
        "modulus" | "mod" => math::modulus::modulus,
        "power" | "pow" => math::power::power,
        "sqrt" | "squareroot" => math::sqrt::sqrt,
        "increase" | "inc" => math::increase::increase,
        "decrease" | "dec" => math::decrease::decrease,

        "equal" | "eq" => math::equal::equal,
        "not_equal" | "neq" => math::not_equal::not_equal,

        "print" | "println" | "printline" | "stdout" => println::println,
        "input" | "stdin" => input::input,
        "exit" | "quit" | "kill" | "terminate" | "close" | "end" => exit::exit,
        _ => return Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no '{}' builtin function",executable.function_name)
            ))
    })
}

