use crate::common::errors::Result;
use crate::common::{
    errors::ChapError,
    executable::{BuiltinFunction, ExecutableLine},
};

mod assign;
mod pass;
mod type_of;
mod utils;

mod bools;
mod collection;
mod control_flow;
mod date_time;
mod debugger;
mod delay;
mod list;
mod math;
mod net;
mod random;
mod std_io;
mod strings;
mod type_conversion;

pub fn closure_gen(executable: &ExecutableLine) -> Result<BuiltinFunction> {
    let debug_mode = executable.function_name.ends_with('?');
    let function_name = executable
        .function_name
        .clone()
        .to_lowercase()
        .replace([' ', '_', '?'], "");

    let function = match function_match(&function_name) {
        Some(f) => f,
        None => {
            return Err(ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!(
                    "there is no function with name: {}",
                    executable.function_name
                ),
            ))
        }
    };

    if debug_mode {
        Ok(debugger::debugger)
    } else {
        Ok(function)
    }
}

pub fn function_match(function_name: &str) -> Option<BuiltinFunction> {
    match function_name {
        "assign" => Some(assign::assign),
        "jump" => Some(control_flow::jump::jump),
        "jumpif" => Some(control_flow::jump_if::jump_if),
        "jumpifnot" => Some(control_flow::jump_if_not::jump_if_not),
        "jumpifequal" | "jeq" => Some(control_flow::jump_if_equal::jump_if_equal),
        "jumpifnotequal" | "jneq" => Some(control_flow::jump_if_not_equal::jump_if_not_equal),
        "newtag" => Some(control_flow::new_tag::new_tag),
        "add" => Some(math::add::add),
        "addmany" | "addall" => Some(math::add_many::add_many),
        "minus" => Some(math::minus::minus),
        "multiply" => Some(math::multiply::multiply),
        "divide" => Some(math::divide::divide),
        "modulus" | "mod" => Some(math::modulus::modulus),
        "power" | "pow" => Some(math::power::power),
        "sqrt" | "squareroot" => Some(math::sqrt::sqrt),
        "increase" | "inc" => Some(math::increase::increase),
        "decrease" | "dec" => Some(math::decrease::decrease),
        "equal" | "eq" => Some(bools::equal::equal),
        "notequal" | "neq" => Some(bools::not_equal::not_equal),
        "and" => Some(bools::and::and),
        "or" => Some(bools::or::or),
        "xor" => Some(bools::xor::xor),
        "not" => Some(bools::not::not),
        "gt" | "greaterthan" => Some(bools::greater_than::greater_than),
        "gte" | "greaterthanequal" => Some(bools::greater_than_equal::greater_than_equal),
        "lt" | "lessthan" => Some(bools::less_than::less_than),
        "lte" | "lessthanequal" => Some(bools::less_than_equal::less_than_equal),
        "concat" | "cat" => Some(strings::contact::concat),
        "repeat" => Some(strings::repeat::repeat),
        "length" | "len" => Some(strings::length::length),
        "contains" | "has" => Some(strings::contains::contains),
        "slice" | "substring" => Some(strings::slice::slice),
        "charat" => Some(strings::char_at::char_at),
        "toupper" | "uppercase" => Some(strings::to_upper::to_upper),
        "tolower" | "lowercase" => Some(strings::to_lower::to_lower),
        "trim" => Some(strings::trim::trim),
        "insert" | "push" => Some(collection::insert::insert),
        "get" | "at" => Some(collection::get::get),
        "pop" => Some(list::pop::pop),
        "last" => Some(list::last::last),
        "includes" | "in" => Some(collection::has::has),
        "removeat" | "rmat" => Some(list::remove_at::remove_at),
        "removeitem" | "rmit" => Some(collection::remove::remove),
        "indexof" => Some(list::index_of::index_of),
        "dump" | "dumpmemory" => Some(debugger::dump::dump),
        "typeof" | "type" => Some(type_of::type_of),
        "tostring" | "tostr" => Some(type_conversion::to_string::to_string),
        "tofloat" => Some(type_conversion::to_float::to_float),
        "toint" => Some(type_conversion::to_int::to_int),
        "tomap" => Some(type_conversion::to_map::to_map),
        "now" | "nowsec" | "unixtime" => Some(date_time::now::now_sec),
        "waitmil" | "waitmillis" => Some(delay::wait_millis::wait_millis),
        "waitsec" | "waitseconds" => Some(delay::wait_second::wait_second),
        "waitmin" | "waitminutes" => Some(delay::wait_minute::wait_minute),
        "waithour" | "delayhour" => Some(delay::wait_hour::wait_hour),
        "print" | "show" | "stdout" => Some(std_io::println::println),
        "input" | "stdin" => Some(std_io::input::input),
        "exit" | "quit" | "kill" | "end" => Some(std_io::exit::exit),
        "pass" | "nop" | "noop" => Some(pass::pass),
        "httpget" => Some(net::http_get::http_get),

        // random functions not working in wasm
        "randomnumber" | "randnum" => Some(random::random_number::random_number),
        "randomstring" | "randstr" => Some(random::random_string::random_string),
        "randombool" | "randbool" => Some(random::random_bool::random_bool),
        "randomchoice" | "randchoice" => Some(random::random_choice::random_choice),
        _ => None,
    }
}
