use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn trim(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    let result = match p1 {
        DataType::String(s) => DataType::String(s.trim().to_string()),
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!(
                    "{} function input param should be string",
                    executable.function_name
                ),
            ));
        }
    };

    returns(runtime, executable, result)
}
