use crate::runtime::builtin_function::utils::{param_to_datatype, param_to_datatype_mut};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn insert(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?
        .clone();
    let p1 = param_to_datatype_mut(&mut (*runtime), executable.params.get(0), executable.line_number)?;

    if let DataType::List(x) = p1{
        x.push(p2);
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, "insert first param should be a list".to_string()));
    }
    
    Ok(())
}