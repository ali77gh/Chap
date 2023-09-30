use crate::{runtime::{runtime::Runtime, builtin_function::utils::param_to_datatype}, common::{executable::ExecutableLine, errors::ChapError}};
use crate::common::errors::Result;
use crate::common::data_type::DataType;
use crate::runtime::builtin_function::control_flow::jump::jump;

// this function can't jump to a tag that is not added to runtime.executables
pub fn jump_if_not(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let b = match p2 {
        DataType::Bool(b) => b,
        _ => {
            return Err(
                ChapError::runtime_with_msg(executable.line_number, "jump_if_not function needs bool as second param".to_string())
            );
        }
    };

    if !b { 
        jump(runtime,executable)?;
    } 
    Ok(())
}