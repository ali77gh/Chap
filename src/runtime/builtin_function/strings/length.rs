use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;


pub fn length(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    let p1 = p1.to_string();
    let result = DataType::Int(p1.len().try_into().unwrap());

    returns(runtime, executable, result)
}
