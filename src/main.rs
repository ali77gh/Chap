

mod parser;
mod executable_line;
mod runtime;

use parser::parse;
use executable_line::ExecutableLine;
use runtime::runtime::Runtime;

use std::env;
use std::fs;

fn main() {
    

    let source = read_source_code();

    let lines = parse(&source);

    let mut runtime = Runtime::new();
    
    println!("{:?}", lines);

    runtime.run_lines(lines);

}

fn read_source_code() -> String {
    if let Some(file_name) = env::args().nth(1) {
        let contents = fs::read_to_string(file_name)
            .expect("err while reading source file");
        contents
    }else{
        panic!("pass a source file\n chap myScript.chp")
    }
}
