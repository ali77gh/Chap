use crate::{
    builtin_function::utils::{param_to_datatype, returns},
    common::{
        data_type::DataType,
        errors::{ChapError, Result},
        executable::ExecutableLine,
    },
    runtime::Runtime,
};

pub fn to_json(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    match p1 {
        DataType::Map(map) => {
            // need to handle null values properly, right now
            // Chap doesnt support Null value, so we case it
            // to a string
            let parsed = serde_json::to_string(map).map_err(|_| {
                ChapError::runtime_with_msg(
                    executable.line_number,
                    format!("failed to parse map to json string"),
                )
            })?;

            returns(runtime, executable, DataType::String(parsed))
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!("first parameter must be a map, got {}", p1.type_name()),
            ))
        }
    }
}
