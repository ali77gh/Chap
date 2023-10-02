use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;
use crate::runtime::builtin_function::math::add::add_data_types;

pub fn add_many(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let mut sum = DataType::Int(0);
    for param in &executable.params{
        let new_value = param_to_datatype(runtime, Some(param), executable.line_number)?;
        sum = add_data_types(&sum, new_value)?;
    }

    returns(runtime, executable, sum)   
}