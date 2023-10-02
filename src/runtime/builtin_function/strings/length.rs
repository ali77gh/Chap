use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn length(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    let p1 = p1.to_string();
    let result = DataType::Int(p1.len().try_into().unwrap());

    match &executable.output_var{
        Some(x) => {
            runtime.variables.insert(x.clone(), result);
            Ok(())
        },
        None => Err(
            ChapError::runtime_with_msg(executable.line_number, "length function needs output variable".to_string())
        ),
    }
}


