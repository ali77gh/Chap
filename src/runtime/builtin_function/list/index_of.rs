use crate::runtime::builtin_function::utils::{returns, param_to_datatype};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn index_of(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let result = if let DataType::List(list) = p1{
        if let Some(index) = list.iter().position(|x| *x == *p2) {
            i32::try_from(index).unwrap_or(-1) + 1
        }else {
            -1
        } 
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, format!("correct form of {0} function: <list>, <item> -> {0}", executable.function_name)));
    };

    let result = DataType::Int(result);
    returns(runtime, executable, result)
}