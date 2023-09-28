use std::{io::{self, Write}, process::exit};

use crate::{
    parser::parser::Parser,
    preprocessor::preprocessor::Preprocessor,
    runtime::runtime::Runtime, common::errors::ErrorType
};


pub fn start_rpel(){

    // initialize
    let mut preprocessor = Preprocessor::new();

    let parser = Parser::new();

    let mut runtime = Runtime::new(|msg|{
        println!("{}", msg);
    },||{
        input()
    });

    loop {
        print!("-> ");
        let _ = io::stdout().flush();
        let source = input(); 
        let lines = preprocessor.on_new_line(source);
        for line in lines{
            let el = parser.on_new_line(line);
            match el {
                Err(err) => err.show_warning(),
                Ok(el) => {
                    if let Err(e)=runtime.on_new_line(el){
                        e.exit_with_error();
                    }
                    'inner: loop {
                        if let Err(err) = runtime.execution_cycle(){
                            match err.err_type {
                                ErrorType::NothingToExecute => break 'inner,
                                ErrorType::Stop => exit(0),
                                _=> {
                                    err.show_warning();
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

fn input() -> String{
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        buffer
}