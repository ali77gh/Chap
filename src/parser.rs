
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
    
    ExecutableLine::Noting
}