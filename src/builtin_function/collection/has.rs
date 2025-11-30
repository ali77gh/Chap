use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn has(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let mut result = DataType::Bool(false);

    match p1 {
        DataType::List(list) => {
            for item in list {
                if item == p2 {
                    result = DataType::Bool(true);
                    break;
                }
            }
        }
        DataType::Map(map) => {
            let p2 = if let DataType::String(s) = p2 {
                s
            } else {
                return Err(ChapError::runtime_with_msg(
                    executable.line_number,
                    "second param should be a string".to_string(),
                ));
            };

            for (key, _) in map {
                if key == p2 {
                    result = DataType::Bool(true);
                    break;
                }
            }
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                "has first param should be a list or map".to_string(),
            ));
        }
    }

    returns(runtime, executable, result)
}
