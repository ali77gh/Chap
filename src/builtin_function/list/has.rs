use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn has(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let mut result = DataType::Bool(false);
    if let DataType::List(list) = p1 {
        for item in list {
            if item == p2 {
                result = DataType::Bool(true);
                break;
            }
        }
    } else {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            "has first param should be a list".to_string(),
        ));
    }

    returns(runtime, executable, result)
}
