use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn char_at(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let (string_value, index) = match (&p1, &p2) {
        (DataType::String(s), DataType::Int(i)) => (s, *i),
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                format!(
                    "{} function requires a string as first parameter and an integer as second parameter",
                    executable.function_name
                ),
            ));
        }
    };

    // Convert usize index to i32 for bounds checking
    if index < 0 {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            format!(
                "Index {} is negative. Index must be non-negative",
                index
            ),
        ));
    }

    let index = index as usize;
    
    // Check if index is within bounds
    if index >= string_value.len() {
        return Err(ChapError::runtime_with_msg(
            executable.line_number,
            format!(
                "Index {} is out of bounds for string of length {}",
                index,
                string_value.len()
            ),
        ));
    }

    // Get the character at the specified index
    let chars: Vec<char> = string_value.chars().collect();
    let result_char = chars[index];
    let result = DataType::String(result_char.to_string());

    returns(runtime, executable, result)
}
