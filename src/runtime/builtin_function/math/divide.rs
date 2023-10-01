use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn divide(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    let result = divide_data_types(p1,p2)?;
    
    match &executable.output_var{
        Some(x) => {
            runtime.variables.insert(x.clone(), result);
            Ok(())
        },
        None => Err(
            ChapError::runtime_with_msg(executable.line_number, "add function needs output variable".to_string())
        ),
    }
}

fn divide_data_types(dt1: &DataType, dt2: &DataType) -> Result<DataType>{
    match (dt1, dt2) {
        (DataType::Int(x1), DataType::Int(x2)) => Ok(DataType::Float(f64::from(*x1)/f64::from(*x2))),
        (DataType::Int(x1), DataType::Float(x2)) => Ok(DataType::Float(f64::from(*x1)/x2)),
        (DataType::Float(x1), DataType::Int(x2)) => Ok(DataType::Float(x1 / f64::from(*x2))),
        (DataType::Float(x1), DataType::Float(x2)) => Ok(DataType::Float(x1/x2)),
        _=> {
            Err(
                ChapError::runtime_with_msg(0, "divide function works only with numbers int and float".to_string())
            )
        }
    }
}
