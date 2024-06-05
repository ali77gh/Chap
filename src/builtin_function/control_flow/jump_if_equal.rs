use crate::builtin_function::control_flow::jump::jump;
use crate::common::errors::Result;
use crate::runtime::Runtime;
use crate::{builtin_function::utils, common::executable::ExecutableLine};

pub fn jump_if_equal(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = utils::param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;
    let p2 = utils::param_to_datatype(runtime, executable.params.get(2), executable.line_number)?;

    if p1 == p2 {
        // Datatype implements PartialEq
        jump(runtime, executable)?;
    }

    Ok(())
}
