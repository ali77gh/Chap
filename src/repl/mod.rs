use std::process::exit;

use crate::{
    parser::Parser,
    preprocessor::Preprocessor,
    runtime::Runtime, common::errors::ErrorType
};

use rustyline::DefaultEditor;


pub fn start_rpel(){

    // initialize
    let mut preprocessor = Preprocessor::default();

    let parser = Parser;

    let mut reader = DefaultEditor::new().unwrap(); // TODO: handle error

    let mut runtime = Runtime::new(|msg|{
        println!("{}", msg);
    },||{
        String::from("")
    });

    loop {
        // let source: String;
        let source = match reader.readline("-> ") {
            Ok(line) => {
                let _ = reader.add_history_entry(line.clone());
                line
            },
            Err(_) => break,
        };
        let lines = preprocessor.on_new_line(source);
        match lines{
            Ok(lines) => {
                for t in lines{
                    let el = parser.on_new_line(t);
                    match el {
                        Err(err) => err.show_warning(),
                        Ok(el) => {
                            if let Err(e)=runtime.on_new_line(el){
                                e.show_warning();
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
            },
            Err(e) => {
                e.show_warning();
            },
        }
            
    }
}
