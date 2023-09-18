
pub enum ErrorType {
    Syntax,
    StaticAnalyzer,
    Runtime,
}

impl ErrorType {
    pub fn to_string(&self) -> &str{
        match *self {
            ErrorType::Syntax => "syntax",
            ErrorType::StaticAnalyzer => "static analyzer",
            ErrorType::Runtime => "runtime",
        }
    }
}

pub struct ChapError{
    line_number: u32,
    msg: Option<String>,
    err_type: ErrorType
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

    pub fn exit_with_error(&self){
        println!("{}", self.error_message());
        panic!();
    }

    // on the REPL mode we should not shut the whole interperter down 
    pub fn show_warning(&self){ 
        println!("{}", self.error_message());
    }

    fn error_message(&self) -> String{
        let mut result = String::new();
        result.push_str("\n");
        result.push_str(&format!("\t{} Error on line: {}", self.err_type.to_string(), self.line_number));
        // result.push_str(&format!("\tyour code: '{}' ",""));//TODO
        if let Some(msg) = self.msg{
            result.push_str(&format!("\terror message: {} ",msg));
        }
        result.push_str("\n");
        result
    }

}
