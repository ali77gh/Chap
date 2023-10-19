use crate::runtime::builtin_function::utils::{param_to_datatype_mut, param_to_datatype};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn remove(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?
        .clone();
    let p1 = param_to_datatype_mut(&mut (*runtime), executable.params.get(0), executable.line_number)?;

    if let DataType::List(list) = p1{
        if let Some(index) = list.iter().position(|x| *x == p2) {
            list.remove(index);
        } // ignore if item not founded
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, format!("correct form of {0} function: <list>, <item> -> {0}", executable.function_name)));
    }

    Ok(())
}