#![allow(dead_code)]

#[derive(PartialEq, Debug)]
pub enum ErrorType {
    Syntax,
    StaticAnalyzer,
    Runtime,
    NothingToExecute,
    Stop,
}

impl ErrorType {
    pub fn to_string(&self) -> &str{
        match *self {
            ErrorType::Syntax => "syntax",
            ErrorType::StaticAnalyzer => "static analyzer",
            ErrorType::Runtime => "runtime",
            ErrorType::NothingToExecute => "no more lines to execute",
            ErrorType::Stop => "stop",
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ChapError{
    line_number: u32,
    msg: Option<String>,
    pub err_type: ErrorType
}

pub type Result<T> = core::result::Result<T, ChapError>;


impl ChapError {
    
    pub fn syntax_with_msg(line_number: u32, msg: String) -> Self {
        Self { line_number, msg: Some(msg), err_type: ErrorType::Syntax }
    }
    pub fn static_analyzer_with_msg(line_number: u32, msg: String) -> Self {
        Self { line_number, msg: Some(msg), err_type: ErrorType::StaticAnalyzer }
    }
    pub fn runtime_with_msg(line_number: u32, msg: String) -> Self {
        Self { line_number, msg: Some(msg), err_type: ErrorType::Runtime }
    }

    pub fn syntax(line_number: u32) -> Self {
        Self { line_number, msg: None, err_type: ErrorType::Syntax }
    }
    pub fn static_analyzer(line_number: u32) -> Self {
        Self { line_number, msg: None, err_type: ErrorType::StaticAnalyzer }
    }
    pub fn runtime(line_number: u32) -> Self {
        Self { line_number, msg: None, err_type: ErrorType::Runtime }
    }
    pub fn no_more_line() -> Self{
        Self { line_number: 0, msg: None, err_type: ErrorType::NothingToExecute }
    }
    pub fn stop() -> Self{
        Self { line_number: 0, msg: None, err_type: ErrorType::Stop }
    }

    pub fn exit_with_error(&self){
        println!("{}", self.error_message());
        panic!();
    }

    // on the REPL mode we should not shut the whole interperter down 
    pub fn show_warning(&self){ 
        println!("{}", self.error_message());
    }

    pub fn error_message(&self) -> String{
        let mut result = String::new();
        result.push_str(&format!("\t{} Error on line: {}", self.err_type.to_string(), self.line_number));
        if let Some(msg) = &self.msg{
            result.push_str(&format!("\terror message: {} ",msg));
        }
        result
    }

}
