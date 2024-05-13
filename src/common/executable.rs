use super::param::Param;
use crate::builtin_function::closure_gen;
use crate::common::errors::Result;
use crate::runtime::Runtime;

pub type BuiltinFunction = fn(&mut Runtime, &ExecutableLine) -> Result<()>;

#[derive(PartialEq, Debug, Clone)]
pub struct ExecutableLine {
    pub line_number: u32, // for error throw information
    pub function_name: String,
    pub params: Vec<Param>,
    pub output_var: Option<String>,
    pub closure: BuiltinFunction,
}

impl ExecutableLine {
    pub fn new(
        line_number: u32,
        function_name: String,
        params: Vec<Param>,
        output_var: Option<String>,
    ) -> Self {
        Self {
            line_number,
            function_name,
            params,
            output_var,
            closure: |_, _| Ok(()),
        }
    }

    pub fn exit() -> Self {
        ExecutableLine::new(0, "exit".to_string(), vec![], None)
    }

    pub fn bind_closure(&mut self) -> Result<()> {
        self.closure = closure_gen(self)?;
        Ok(())
    }
}
