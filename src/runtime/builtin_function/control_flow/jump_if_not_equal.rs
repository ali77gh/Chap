
use crate::{runtime::{runtime::Runtime, builtin_function::utils}, common::{executable::ExecutableLine, errors::ChapError}};
use crate::common::errors::Result;
use crate::common::data_type::DataType;
use crate::runtime::builtin_function::control_flow::jump::jump;

pub fn jump_if_equal(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = utils::param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;
    let p2 = utils::param_to_datatype(runtime, executable.params.get(2), executable.line_number)?;

    let are_equal = match (p1, p2) {
        (DataType::String(x1), DataType::String(x2)) => x1 == x2,
        (DataType::Int(x1), DataType::Int(x2)) => x1 == x2,
        (DataType::Float(x1), DataType::Float(x2)) => x1 == x2,
        (DataType::Bool(x1), DataType::Bool(x2)) => x1 == x2,
        _ => {
            return Err(ChapError::runtime_with_msg(executable.line_number, "equal can not apply to diffrent types".to_string()));
        }
    };

    if !are_equal{
        jump(runtime, executable)?;
    }

    Ok(())    
}