use crate::common::{executable::{BuiltinFunction, ExecutableLine}, errors::ChapError};
use crate::common::errors::Result;

mod assign;
mod utils;
mod type_of;

mod control_flow;
mod math;
mod std;
mod bools;
mod strings;
mod type_conversion;
mod date_time;
mod delay;
mod debugger;
mod error_handling;
mod list;

pub fn closure_gen(executable: &ExecutableLine) -> Result<BuiltinFunction>{

    let debug_mode = executable.function_name.ends_with("?");
    let function_name = executable.function_name
        .clone()
        .to_lowercase()
        .replace([' ', '_', '?'], "");

    
    let function = match function_match(&function_name){
        Some(f) => f,
        None => return Err(
            ChapError::static_analyzer_with_msg(
                executable.line_number,
                format!("there is no function with name: {}", executable.function_name)
            )
        )
    };

    if debug_mode{
        Ok(debugger::debugger)
    }else {
        Ok(function)
    }
}


#[cfg(target_family = "unix")]
pub fn function_match(function_name: &str) -> Option<BuiltinFunction>{
    if let Some(f) = common_functions(function_name){
        Some(f)
    }else {
        random_functions(function_name)
    }
}

#[cfg(target_family = "windows")]
pub fn function_match(function_name: &str) -> Option<BuiltinFunction>{
    if let Some(f) = common_functions(function_name){
        return Some(f);
    }else if let Some(f) = random_functions(function_name) {
        return Some(f);
    }else{
        None
    }
}

#[cfg(target_family = "wasm")]
pub fn function_match(function_name: &str) -> Option<BuiltinFunction>{
    if let Some(f) = common_functions(function_name){
        return Some(f);
    }else{
        None
    }
}


pub fn common_functions(function_name: &str) -> Option<BuiltinFunction>{

    match function_name {
        "assign" => Some(assign::assign),
        "jump" =>Some( control_flow::jump::jump),
        "jumpif" =>Some( control_flow::jump_if::jump_if),
        "jumpifnot" =>Some( control_flow::jump_if_not::jump_if_not),
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
        "not" => Some(bools::not::not),
        "gt" | "greaterthan" => Some(bools::greater_than::greater_than),
        "lt" | "lessthan" => Some(bools::less_than::less_than),
        "concat" | "cat" => Some(strings::contact::concat),
        "repeat"  => Some(strings::repeat::repeat),
        "length" | "len"  => Some(strings::length::length),
        "contains" | "has"  => Some(strings::contains::contains),
        "slice" | "substring"  => Some(strings::slice::slice),
        "insert" | "push" => Some(list::insert::insert),
        "get" | "at" => Some(list::get::get),
        "pop" => Some(list::pop::pop),
        "last" => Some(list::last::last),
        "includes" | "in" => Some(list::has::has),
        "removeat" | "rmat" => Some(list::remove_at::remove_at),
        "removeitem" | "rmit" => Some(list::remove::remove),
        "indexof"  => Some(list::index_of::index_of),
        "dump" | "dumpmemory" => Some(debugger::dump::dump),
        "typeof" | "type" => Some(type_of::type_of),
        "tostring" | "tostr" => Some(type_conversion::to_string::to_string),
        "tofloat"  => Some(type_conversion::to_float::to_float),
        "toint"  => Some(type_conversion::to_int::to_int),
        "now" | "nowsec" | "unixtime"  => Some(date_time::now::now_sec),
        "waitmil" | "waitmillis" => Some(delay::wait_millis::wait_millis),
        "waitsec" | "waitseconds" => Some(delay::wait_second::wait_second),
        "waitmin" | "waitminutes" => Some(delay::wait_minute::wait_minute),
        "waithour" | "delayhour" => Some(delay::wait_hour::wait_hour),
        "print" | "show" | "stdout" => Some(std::println::println),
        "input" | "stdin" => Some(std::input::input),
        "exit" | "quit" | "kill" | "end" => Some(std::exit::exit),
        _=>{
            None
        }
    }
}

// platform specific functions
#[cfg(not(target_family = "wasm"))]
mod random;
#[cfg(not(target_family = "wasm"))]
pub fn random_functions(function_name: &str) -> Option<BuiltinFunction>{

    match function_name {
        "randomnumber" | "randnum" => Some(random::random_number::random_number),
        "randomstring" | "randstr" => Some(random::random_string::random_string),
        "randombool" | "randbool" => Some(random::random_bool::random_bool),
        "randomchoice" | "randchoice" => Some(random::random_choice::random_choice),
        _=>{
            None
        }
    }
}
