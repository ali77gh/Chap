use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use crate::runtime::builtin_function::math::add::add_data_types;

pub fn add_many(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{


    let mut sum = DataType::Int(0);
    for param in &executable.params{
        let new_value = param_to_datatype(runtime, Some(param), executable.line_number)?;
        sum = add_data_types(&sum, new_value)?;
    }
    
    match &executable.output_var{
        Some(x) => {
            runtime.variables.insert(x.clone(), sum);
            Ok(())
        },
        None => Err(
            ChapError::runtime_with_msg(executable.line_number, "add_many function needs output variable".to_string())
        ),
    }
}