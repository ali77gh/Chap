use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn sqrt(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;

    let result = sqrt_data_type(p1)?;
    returns(runtime, executable, result)
}

fn sqrt_data_type(dt: &DataType) -> Result<DataType> {
    match dt {
        DataType::Int(x) => Ok(DataType::Float(f64::sqrt(f64::from(*x)))),
        DataType::Float(x) => Ok(DataType::Float(f64::sqrt(*x))),
        _ => Err(ChapError::runtime_with_msg(
            0,
            "sqrt function works only with numbers int and float".to_string(),
        )),
    }
}
