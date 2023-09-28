
use crate::{runtime::runtime::Runtime, common::{executable::ExecutableLine, errors::ChapError, param::Param}};
use crate::common::errors::Result;
use crate::common::data_type::DataType;
use crate::runtime::builtin_function::control_flow::jump::jump;

// this function can't jump to a tag that is not added to runtime.executables
pub fn jump_if(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let dt = match executable.params.get(1){
        Some(v) => match v {
            Param::Variable(v) => match runtime.variables.get(v){
                Some(v) => v,
                None => return Err(
                    ChapError::runtime_with_msg(executable.line_number, format!("variable {} not exist",v))
                ),
            },
            Param::Value(v) => v,
            Param::Tag(_) => return Err(
                ChapError::runtime_with_msg(executable.line_number, "jump_if function needs bool as second param".to_string())
            ),
        },
        None => return Err(
            ChapError::runtime_with_msg(executable.line_number, "jump_if function needs bool as second param".to_string())
        )
    };
    let b = match dt {
        DataType::Bool(b) => b,
        _ => {
            return Err(
                ChapError::runtime_with_msg(executable.line_number, "jump_if function needs bool as second param".to_string())
            );
        }
    };

    if !b{ return Ok(()); } // prevent jump if b is false

    jump(runtime,executable)
    
}


// TODO tests