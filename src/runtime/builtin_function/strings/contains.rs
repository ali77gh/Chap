use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn contains(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let p1 = p1.to_string();
    let p2 = p2.to_string();

    let result = DataType::Bool(p1.contains(p2.as_str()));

    match &executable.output_var{
        Some(x) => {
            runtime.variables.insert(x.clone(), result);
            Ok(())
        },
        None => Err(
            ChapError::runtime_with_msg(executable.line_number, "contains function needs output variable".to_string())
        ),
    }
}


