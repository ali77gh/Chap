use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::runtime::builtin_function::utils::{param_to_datatype_mut, returns};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn pop(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype_mut(
        &mut (*runtime),
        executable.params.first(),
        executable.line_number,
    )?;

    if let DataType::List(x) = p1 {
        let last = x.last();
        if let Some(last) = last {
            let result = last.clone();
            x.pop();
            returns(runtime, executable, result)
        } else {
            Err(ChapError::runtime_with_msg(
                executable.line_number,
                "list is empty".to_string(),
            ))
        }
    } else {
        Err(ChapError::runtime_with_msg(
            executable.line_number,
            "insert first param should be a list".to_string(),
        ))
    }
}
