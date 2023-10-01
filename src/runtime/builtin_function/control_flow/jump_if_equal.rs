use crate::{runtime::{Runtime, builtin_function::utils}, common::executable::ExecutableLine};
use crate::common::errors::Result;
use crate::runtime::builtin_function::control_flow::jump::jump;

pub fn jump_if_equal(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = utils::param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;
    let p2 = utils::param_to_datatype(runtime, executable.params.get(2), executable.line_number)?;

    if p1 == p2{ //Datatype impelements PartialEq
        jump(runtime, executable)?;
    }

    Ok(())    
}