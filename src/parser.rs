
use crate::ExecutableLine;


pub fn parse(content: &str) -> Vec<ExecutableLine> {
    let raw_lines = content.split("\n");

    let mut result = vec![];

    for (line_number,raw_line) in raw_lines.enumerate() {
        result.push(parse_line(&raw_line,line_number));
    }

    result
}

fn parse_line(line: &str, line_number: usize) -> ExecutableLine {
    
    let line_number = line_number+1;

    let mut line = line.trim();
    line = line.split("//").nth(0).unwrap_or(line); //remove all comments
    
    if line.is_empty(){ return ExecutableLine::Noting; }

    ExecutableLine::SyntaxError(format!("syntax error in line {}",line_number).to_string())
}