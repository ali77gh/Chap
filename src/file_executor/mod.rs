
use std::io;
use std::fs::read_to_string;
use std::process::exit;

use crate::common::errors::{Result, ErrorType};
use crate::preprocessor::Preprocessor;
use crate::parser::Parser;
use crate::runtime::Runtime;

pub fn file_executor(file_name: &str) -> Result<()>{

    // initialize
    let mut preprocessor = Preprocessor::default();

    let mut parser= Parser::default();

    let mut runtime = Runtime::new(|msg|{
        println!("{}", msg);
    },||{
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        buffer = buffer.replace('\n', "").trim().to_string();
        buffer
    });


    for line in read_to_string(file_name).unwrap().lines() {
        let ls = preprocessor.on_new_line(line.to_string());
        match ls{
            Ok(ls) => {
                for line in ls{
                    let e = parser.on_new_line(line);
                    match e {
                        Ok(els) => {
                            for el in els{
                                if let Err(e)=runtime.on_new_line(el){
                                    e.exit_with_error();
                                }
                            }
                        },
                        Err(err) => {
                            err.exit_with_error();
                        },
                    }
                }
            },
            Err(e) => e.exit_with_error(),
        }
    }

    loop {
        if let Err(err) = runtime.execution_cycle(){
            match err.err_type {
                ErrorType::NothingToExecute | ErrorType::Stop => exit(0),
                _=> {
                    err.exit_with_error();
                }
            }
        }
    }

}