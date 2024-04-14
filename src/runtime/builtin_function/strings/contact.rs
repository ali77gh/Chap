use crate::common::data_type::DataType;
use crate::common::errors::Result;
use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn concat(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let mut result = String::new();
    for param in &executable.params {
        let dt = param_to_datatype(runtime, Some(param), executable.line_number)?;
        result.push_str(dt.to_string().as_str());
    }

    returns(runtime, executable, DataType::String(result))
}
