use crate::common::data_type::DataType;
use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use std::{thread, time};

pub fn wait_millis(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{
    
    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    if let DataType::Int(x) = p1{
        let ten_millis = time::Duration::from_millis((*x).try_into().unwrap());
        thread::sleep(ten_millis);
    }else{
        return Err(
            ChapError::runtime_with_msg(executable.line_number, format!("function {} needs int param as input", executable.function_name))
        );
    }

    Ok(())
}
