use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn minus(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let result = minus_data_types(p1,p2)?;
    
    returns(runtime, executable, result)
}

fn minus_data_types(dt1: &DataType, dt2: &DataType) -> Result<DataType>{
    match (dt1, dt2) {
        (DataType::Int(x1), DataType::Int(x2)) => Ok(DataType::Int(x1-x2)),
        (DataType::Int(x1), DataType::Float(x2)) => Ok(DataType::Float(f64::from(*x1)-x2)),
        (DataType::Float(x1), DataType::Int(x2)) => Ok(DataType::Float(x1 - f64::from(*x2))),
        (DataType::Float(x1), DataType::Float(x2)) => Ok(DataType::Float(x1-x2)),
        _=> {
            Err(
                ChapError::runtime_with_msg(0, "minus function works only with numbers int and float".to_string())
            )
        }
    }
}
