use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn not_equal(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    if let Some(var_name) = &executable.output_var{
        runtime.variables.insert(var_name.clone(), DataType::Bool(p1 != p2)); //Datatype impelements PartialEq
    }else{
        return Err(
            ChapError::runtime_with_msg(executable.line_number, "equal function needs output variable".to_string())
        );
    }
    Ok(())
}