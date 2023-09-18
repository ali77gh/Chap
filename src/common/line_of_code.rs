

#[derive(Debug)]
pub struct LineOfCode{
    pub line_number: u32,
    pub code: String
}

impl LineOfCode {
    pub fn new(line_number: u32, code: String) -> Self{
        Self{line_number, code}
    }
}