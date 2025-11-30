use crate::builtin_function::utils::{param_to_datatype, param_to_datatype_mut};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn remove(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?.clone();
    let p1 = param_to_datatype_mut(
        &mut (*runtime),
        executable.params.first(),
        executable.line_number,
    )?;

    match p1 {
        DataType::List(list) => {
            if let Some(index) = list.iter().position(|x| *x == p2) {
                list.remove(index);
            } // ignore if item not founded
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

            map.remove(&p2);
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!(
                    "correct form of {0} function: <list | map>, <item | key> -> {0}",
                    executable.function_name
                ),
            ));
        }
    }

    Ok(())
}
