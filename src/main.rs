
mod common; // language common defenitions

//phases
mod preprocessor;    // phase 1
mod parser;          // phase 2
mod static_analyzer; // phase 3
mod optimizer;       // phase 4
mod runtime;         // phase 5

// execution phase 5
mod repl;
mod file_executor; 

use std::env;
use crate::common::errors::Result;
use crate::file_executor::file_executor;

fn main() ->Result<()> {

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            // TODO repl
        },
        2 => {
            file_executor(args.get(1).unwrap())?;
        },
        _ => {
            show_help();
        }
    }
    Ok(())
}

pub fn show_help(){
    println!("┌───────────────────────────────────────────────┐");
    println!("│                Chap Language                  │");
    println!("│                                               │");
    println!("│ https://github.com/ali77gh/Chap               │");
    println!("│                                               │");
    println!("│ How to:                                       │");
    println!("│     run script file:                          │");
    println!("│         $ chap <file_name>                    │");
    println!("│     run repl mode:                            │");
    println!("│         $ chap                                │");
    println!("│                                               │");
    println!("│ Options:                                      │");
    println!("│  -h, --help                                   │");
    println!("│  -v, --version                                │");
    println!("│  -u, --update                                 │");
    println!("│                                               │");
    println!("└───────────────────────────────────────────────┘");
              
 }