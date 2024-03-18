use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn length(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    let result = match p1 {
        DataType::String(s) => s.len(),
        DataType::List(l) => l.len(),
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!(
                    "{} function input param should be string or list",
                    executable.function_name
                ),
            ));
        }
    };
    let result = DataType::Int(result.try_into().unwrap());

    returns(runtime, executable, result)
}
