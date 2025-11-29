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

    match p1 {
        DataType::List(list) => {
            list.push(p2);
        }
        DataType::Map(map) => {
            let p2 = if let DataType::Map(m) = p2 {
                m
            } else {
                return Err(ChapError::runtime_with_msg(
                    executable.line_number,
                    "second param should be a map with one key-value pair".to_string(),
                ));
            };

            for (key, value) in p2 {
                map.insert(key.to_string(), value.clone());
            }
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                "insert first param should be a list or map".to_string(),
            ))
        }
    }

    Ok(())
}
