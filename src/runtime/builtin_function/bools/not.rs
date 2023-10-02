use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn not(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    let result = match p1 {
        DataType::Bool(x) => !x,
        _=>{
            return Err(
                ChapError::runtime_with_msg(executable.line_number, "not function needs bool params".to_string())
            );
        }
    };

    returns(runtime, executable,DataType::Bool(result))
}