
use super::param::Param;

#[derive(PartialEq, Debug, Clone)]
pub struct ExecutableLine{
    pub line_number: u32, // for error throw information
    pub function_name: String,
    pub params: Vec<Param>,
    pub output_var: Option<String>
}

impl ExecutableLine {
    
    pub fn new(
        line_number: u32,
        function_name:String,
        params:Vec<Param>,
        output_var:Option<String>
    ) -> Self{
        Self { line_number, function_name, params, output_var }
    }
}