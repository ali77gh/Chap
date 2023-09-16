

#[derive(Debug)]
pub struct LineOfCode{
    pub line_number: u32,
    pub parsable_code: String,
    pub written_code: String
}

impl LineOfCode {
    pub fn new(line_number: u32, parsable_code: String, written_code: String) -> Self{
        Self{line_number, parsable_code, written_code}
    }
}