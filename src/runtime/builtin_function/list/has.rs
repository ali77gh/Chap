use crate::runtime::builtin_function::utils::{returns, param_to_datatype};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn has(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let mut result = DataType::Bool(false);
    if let DataType::List(list)= p1{
        for item in list{
            if item == p2{
                result = DataType::Bool(true);
                break;
            }
        }
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, "has first param should be a list".to_string()));
    }

    returns(runtime, executable, result)
}