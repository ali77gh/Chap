use crate::{runtime::{runtime::Runtime, builtin_function::utils::param_to_datatype}, common::{executable::ExecutableLine, errors::ChapError}};
use crate::common::errors::Result;
use crate::runtime::builtin_function::control_flow::jump::jump;

pub fn jump_if_not_equal(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(2), executable.line_number)?;

    if p1 != p2{//Datatype impelements PartialEq
        jump(runtime, executable)?;
    }

    Ok(())    
}