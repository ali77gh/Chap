use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn and(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let mut result = true;
    for param in &executable.params{
        let dt = param_to_datatype(runtime, Some(param), executable.line_number)?;
        match dt {
            DataType::Bool(x) => result = result && *x,
            _=>{
                return Err(
                    ChapError::runtime_with_msg(executable.line_number, "and function needs bool params".to_string())
                );
            }
        }
    }

    returns(runtime, executable,DataType::Bool(result))
}