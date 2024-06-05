use crate::builtin_function::utils::{param_to_datatype, param_to_datatype_mut};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn insert(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?.clone();
    let p1 = param_to_datatype_mut(
        &mut (*runtime),
        executable.params.first(),
        executable.line_number,
    )?;

    if let DataType::List(x) = p1 {
        x.push(p2);
    } else {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            "insert first param should be a list".to_string(),
        ));
    }

    Ok(())
}
