use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn to_string(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    let result = p1.to_string();
    returns(runtime, executable, DataType::String(result))
}
