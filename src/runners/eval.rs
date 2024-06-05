use crate::common::errors::{ChapError, ErrorType, Result};
use crate::common::executable::ExecutableLine;
use crate::compile::parser::Parser;
use crate::compile::preprocessor::Preprocessor;
use crate::runtime::Runtime;

pub fn eval<'a>(
    code: String,
    std_out: Box<dyn FnMut(&str) + 'a>,
    std_in: Box<dyn FnMut() -> String + 'a>,
    mut on_error: impl FnMut(ChapError),
) {
    let mut runtime = match make_runtime(code, std_out, std_in) {
        Ok(rt) => rt,
        Err(e) => {
            on_error(e);
            return;
        }
    };

    loop {
        // Safety: make runtime function is adding exit to the end of source code
        if let Err(e) = unsafe { runtime.execution_cycle() } {
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

fn make_runtime<'a>(
    code: String,
    std_out: Box<dyn FnMut(&str) + 'a>,
    std_in: Box<dyn FnMut() -> String + 'a>,
) -> Result<Runtime<'a>> {
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
    // Safety: remove this will cause segmentation fault
    runtime.on_new_line(ExecutableLine::exit())?; // performance improvement (no need to check if there is more lines)
    Ok(runtime)
}

#[cfg(test)]
mod tests {
    use crate::common::errors::ChapError;

    use super::eval;

    #[test]
    fn test_eval() {
        eval(
            "3".to_string(),
            Box::new(|_| {}),
            Box::new(|| "".to_string()),
            |_| {},
        );
    }

    #[test]
    fn test_eval_closure() {
        let input = "INPUT".to_string();
        let mut output = "".to_string();

        eval(
            "input -> $a; $a -> print".to_string(),
            Box::new(|out| {
                output = out.to_string();
            }),
            Box::new(|| {
                return input.clone();
            }),
            |_| {},
        );
        assert_eq!(input, output);
    }

    #[test]
    fn test_eval_error() {
        let mut error: Option<ChapError> = None;
        eval(
            "a".to_string(),
            Box::new(|_| {}),
            Box::new(|| {
                return "".to_string();
            }),
            |e| {
                error = Some(e);
            },
        );

        if let None = error {
            assert!(false, "error not happened");
        }
    }
}
