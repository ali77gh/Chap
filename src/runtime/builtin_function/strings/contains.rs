use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::Result;


pub fn contains(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let p1 = p1.to_string();
    let p2 = p2.to_string();

    let result = DataType::Bool(p1.contains(p2.as_str()));

    returns(runtime, executable, result)
}


