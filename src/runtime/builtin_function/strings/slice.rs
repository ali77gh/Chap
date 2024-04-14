use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn slice(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;
    let p3 = param_to_datatype(runtime, executable.params.get(2), executable.line_number)?;

    let p1 = p1.to_string();
    let result = match (p2, p3) {
        (DataType::Int(from), DataType::Int(to)) => {
            let from = usize::try_from(*from).unwrap();
            let to = usize::try_from(*to).unwrap();
            &p1[(from)..to]
        }
        _ => {
            return Err(ChapError::runtime_with_msg(
                executable.line_number,
                "slice function needs output variable".to_string(),
            ))
        }
    };

    returns(runtime, executable, DataType::String(result.to_string()))
}
