use crate::common::errors::{ChapError, ErrorType, Result};
use crate::common::executable::ExecutableLine;
use crate::compile::parser::Parser;
use crate::compile::preprocessor::Preprocessor;
use crate::runtime::Runtime;

pub fn eval(code: String, std_out: fn(&str), std_in: fn() -> String, on_error: fn(ChapError)) {
    let mut runtime = match make_runtime(code, std_out, std_in) {
        Ok(rt) => rt,
        Err(e) => {
            on_error(e);
            return;
        }
    };

    loop {
        if let Err(e) = runtime.execution_cycle() {
            match e.err_type {
                ErrorType::Stop => {
                    // stop happened when user call exit function (it's not error)
                    return;
                }
                _ => {
                    on_error(e);
                    return;
                }
            }
        }
    }
}

fn make_runtime(code: String, std_out: fn(&str), std_in: fn() -> String) -> Result<Runtime> {
    let mut preprocessor = Preprocessor::default();
    let mut parser = Parser::default();
    let mut runtime = Runtime::new(std_out, std_in);

    for line in code.split('\n') {
        let ls = preprocessor.on_new_line(line.to_string())?;
        for line in ls {
            let els = parser.on_new_line(line)?;
            for el in els {
                runtime.on_new_line(el)?;
            }
        }
    }
    runtime.on_new_line(ExecutableLine::exit())?; // performance improvement (no need to check if there is more lines)
    Ok(runtime)
}

#[cfg(test)]
mod tests {
    use super::eval;

    #[test]
    fn test_eval() {
        eval("3".to_string(), |_| {}, || "".to_string(), |_| {});
    }
}
