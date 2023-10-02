use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn repeat(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let p1 = p1.to_string();
    let mut result = String::new();
    if let DataType::Int(x) = p2 {
        for _ in 0..(*x){
            result.push_str(p1.as_str());
        }
    }else{
        return Err(
            ChapError::runtime_with_msg(executable.line_number, "repeat function second param should be int".to_string())
        );
    }

    returns(runtime, executable, DataType::String(result))
}

