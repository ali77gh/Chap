use crate::common::{line_of_code::LineOfCode, executable::ExecutableLine};

struct Parser {}

impl Parser {
    fn new() -> Self {
        Parser {}
    }

    fn on_new_line(&self, new_line: LineOfCode) -> ExecutableLine{
        let mut it = new_line.code.split("->").into_iter();
        
        match (it.next(), it.next(), it.next()) {
            (Some(chank1), None, None) => todo!(),
            (Some(chank1), Some(chunk2), None) => todo!(),
            (Some(chank1), Some(chunk2), Some(chunk3)) => todo!(),
            _ => todo!(),
        }
    }
}
