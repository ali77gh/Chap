use std::fs::read_to_string;
use std::io;

use crate::runners::eval::eval;

pub fn file_executor(file_name: &str) {
    let code = read_to_string(file_name).unwrap();
    eval(
        code,
        |msg| {
            println!("{}", msg);
        },
        || {
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer).unwrap();
            buffer = buffer.replace('\n', "").trim().to_string();
            buffer
        },
        |e| {
            e.exit_with_error();
        },
    );
}
