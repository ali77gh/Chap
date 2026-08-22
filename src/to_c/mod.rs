//! Chap to C compiler
//!
//! Turns a list of [`ExecutableLine`] (the output of chap's parser) into a
//! self contained compilable C program with a `main` function.
//!
//! Only a subset of chap builtin functions are implemented:
//! assign, print, input, jump, jump_if_not, jeq, jneq, new_tag, increase,
//! mod, add, minus, multiply, divide, sqrt, lt, lte, gt, gte, eq, neq,
//! and, or, xor, not, to int, cat and exit.

mod gen;

use crate::common::errors::Result;
use crate::common::executable::ExecutableLine;
use crate::compile::parser::Parser;
use crate::compile::preprocessor::Preprocessor;

/// Compile chap source code into C code
pub fn compile_to_c(code: &str) -> Result<String> {
    gen::generate(&compile_source(code)?)
}

/// Run chap source through the preprocessor and parser (same pipeline the
/// interpreter uses) and return the executable lines plus a final `exit`
pub fn compile_source(code: &str) -> Result<Vec<ExecutableLine>> {
    let mut preprocessor = Preprocessor::default();
    let mut parser = Parser::default();
    let mut executables = Vec::<ExecutableLine>::new();

    for line in code.split('\n') {
        for line_of_code in preprocessor.on_new_line(line.to_string())? {
            for executable in parser.on_new_line(line_of_code)? {
                executables.push(executable);
            }
        }
    }

    // same as eval: exit at the end so jumps past the last line terminate
    executables.push(ExecutableLine::exit());
    Ok(executables)
}

/// Compile a list of executable lines into C code
#[allow(dead_code)]
pub fn to_c(executables: &[ExecutableLine]) -> Result<String> {
    gen::generate(executables)
}
