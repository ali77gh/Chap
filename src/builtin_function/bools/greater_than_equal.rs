use crate::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::common::errors::{ChapError, Result};
use crate::{common::executable::ExecutableLine, runtime::Runtime};

pub fn greater_than_equal(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()> {
    let p1 = param_to_datatype(runtime, executable.params.first(), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let result = greater_than_equal_data_types(p1, p2)?;

    returns(runtime, executable, result)
}

pub fn greater_than_equal_data_types(dt1: &DataType, dt2: &DataType) -> Result<DataType> {
    match (dt1, dt2) {
        (DataType::Int(x1), DataType::Int(x2)) => Ok(DataType::Bool(x1 >= x2)),
        (DataType::Int(x1), DataType::Float(x2)) => Ok(DataType::Bool(f64::from(*x1) >= *x2)),
        (DataType::Float(x1), DataType::Int(x2)) => Ok(DataType::Bool(*x1 >= f64::from(*x2))),
        (DataType::Float(x1), DataType::Float(x2)) => Ok(DataType::Bool(x1 >= x2)),
        _ => Err(ChapError::runtime_with_msg(
            0,
            "greater_than_equal function works only with numbers int and float".to_string(),
        )),
    }
}
