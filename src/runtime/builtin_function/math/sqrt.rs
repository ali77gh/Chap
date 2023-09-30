use crate::runtime::builtin_function::utils::param_to_datatype;
use crate::common::data_type::DataType;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn sqrt(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;

    let result = sqrt_data_type(p1)?;
    
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

fn sqrt_data_type(dt: &DataType) -> Result<DataType>{
    match dt {
        DataType::Int(x) => Ok(DataType::Float(f64::sqrt(f64::from(*x)))),
        DataType::Float(x) => Ok(DataType::Float(f64::sqrt(*x))),
        _=>{
            Err(
                ChapError::runtime_with_msg(0, "add function works only with numbers int and float".to_string())
            )
        }
    }
}
