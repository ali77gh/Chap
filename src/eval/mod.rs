use crate::common::errors::{ErrorType, ChapError};
use crate::preprocessor::Preprocessor;
use crate::parser::Parser;
use crate::runtime::Runtime;

pub fn eval(code: String, std_out: fn(&str), std_in: fn() -> String, on_exit: fn(), on_error: fn(ChapError)) {

    // initialize
    let mut preprocessor = Preprocessor::default();
    let parser= Parser;
    let mut runtime = Runtime::new(std_out, std_in);

    for line in code.split("\n") {
        let ls = preprocessor.on_new_line(line.to_string());
        for line in ls{
            let e = parser.on_new_line(line);
            match e {
                Ok(el) => {
                    if let Err(e) = runtime.on_new_line(el){
                        on_error(e);
                        return;
                    }
                },
                Err(e) => {
                    on_error(e);
                    return;
                },
            }
            
        }
    }

    loop {
        if let Err(e) = runtime.execution_cycle(){
            match e.err_type {
                ErrorType::NothingToExecute | ErrorType::Stop => {
                    on_exit();
                    return;
                },
                _=> {
                    on_error(e);
                    return;
                }
            }
        }
    }

}


#[cfg(test)]
mod tests{
    use super::eval;

    #[test]
    fn test_eval(){
        eval(
            "3".to_string(), |_|{}, ||{"".to_string()}, ||{}, |_|{});
    }
}