use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn concat(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let mut result = String::new();
    for param in &executable.params{
        let dt = param_to_datatype(runtime, Some(param), executable.line_number)?;
        result.push_str(dt.to_string().as_str());
    }

    match &executable.output_var{
        Some(x) => {
            runtime.variables.insert(x.clone(), DataType::String(result));
            Ok(())
        },
        None => Err(
            ChapError::runtime_with_msg(executable.line_number, "concat function needs output variable".to_string())
        ),
    }
}
