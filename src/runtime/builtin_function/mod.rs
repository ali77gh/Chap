use crate::common::{executable::{BuiltinFunction, ExecutableLine}, errors::ChapError};
use crate::common::errors::Result;

mod assign;

mod control_flow;
mod math;
mod utils;
mod std;
mod bools;
mod strings;
mod random;
mod type_of;
mod type_conversion;
mod date_time;
mod delay;
mod debug;
mod error_handling;

pub fn closure_gen(executable: &ExecutableLine) -> Result<BuiltinFunction>{

    let function_name = executable.function_name
        .clone()
        .to_lowercase()
        .replace([' ', '_'], "");

    let function = match function_name.as_str() {
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

        "equal" | "eq" => bools::equal::equal,
        "not_equal" | "neq" => bools::not_equal::not_equal,
        "and" => bools::and::and,
        "or" => bools::or::or,
        "not" => bools::not::not,
        "gt" | "greaterthan" => bools::greater_than::greater_than,
        "lt" | "lessthan" => bools::less_than::less_than,
        
        "concat" | "cat" => strings::contact::concat,
        "repeat"  => strings::repeat::repeat,
        "length" | "len"  => strings::length::length,
        "contains" | "has"  => strings::contains::contains,
        "slice" | "substring"  => strings::slice::slice,

        "dump" | "dumpmemory" | "showeverything"  => debug::dump::dump,

        "randomnumber" | "randnum" => random::random_number::random_number,
        "randomstring" | "randstr" => random::random_string::random_string,
        "randombool" | "randbool" => random::random_bool::random_bool,
        "randomchoice" | "randchoice" => random::random_choice::random_choice,

        "typeof" | "type" => type_of::type_of,

        "tostring" | "tostr" => type_conversion::to_string::to_string,
        "tofloat"  => type_conversion::to_float::to_float,
        "toint"  => type_conversion::to_int::to_int,

        "now" | "nowsec" | "unixtime"  => date_time::now::now_sec,

        "waitmil" | "delaymil" | "waitmillis" | "delaymillis"  => delay::wait_millis::wait_millis,
        "waitsec" | "delaysec" | "waitseconds" | "delaysecond"  => delay::wait_second::wait_second,
        "waitmin" | "delaymin" | "waitminutes" | "delayminutes"  => delay::wait_minute::wait_minute,
        "waithour" | "delayhour" => delay::wait_hour::wait_hour,

        "print" | "println" | "printline" | "stdout" => std::println::println,
        "input" | "stdin" => std::input::input,
        "exit" | "quit" | "kill" | "terminate" | "close" | "end" => std::exit::exit,
        _ => return Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no '{}' builtin function",executable.function_name)
            ))
    };

    if executable.debug_mode{
        Ok(debug::debugger::debugger)
    }else {
        Ok(function)
    }
}
