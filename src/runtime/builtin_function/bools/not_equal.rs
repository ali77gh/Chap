use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn not_equal(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    returns(runtime, executable, DataType::Bool(p1 != p2))
}
