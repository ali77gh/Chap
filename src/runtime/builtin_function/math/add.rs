use crate::common::data_type::DataType;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use crate::common::param::Param;

pub fn add(runtime: &mut Runtime, executable: ExecutableLine)-> Result<()>{

    let sum = match (executable.params.get(0) , executable.params.get(1)){
        (Some(Param::Value(v1)), Some(Param::Value(v2))) => 
            add_data_types(&v1,&v2)?,
        (Some(Param::Variable(v1)), Some(Param::Variable(v2))) => 
            add_data_types(&get_var(runtime, v1)?, &get_var(runtime, v2)?)?,
        (Some(Param::Value(v1)), Some(Param::Variable(v2))) => 
            add_data_types(&v1, &get_var(runtime, v2)?)?,
        (Some(Param::Variable(v1)), Some(Param::Value(v2))) => 
            add_data_types(&get_var(runtime, v1)?, & v2)?,
        _=> return Err(
            ChapError::runtime_with_msg(executable.line_number, "you should pass two int params to add function".to_string())
        )
    };
    
    match &executable.output_var{
        Some(x) => {
            runtime.variables.insert(x.clone(), sum);
        },
        None => return Err(
            ChapError::runtime_with_msg(executable.line_number, "add function needs output variable".to_string())
        ),
    };

    Ok(())
}

fn add_data_types(dt1: &DataType, dt2: &DataType) -> Result<DataType>{
    match (dt1, dt2) {
        (DataType::Int(x1), DataType::Int(x2)) => Ok(DataType::Int(x1+x2)),
        (DataType::Int(x1), DataType::Float(x2)) => Ok(DataType::Float(f64::from(*x1)+x2)),
        (DataType::Float(x1), DataType::Int(x2)) => Ok(DataType::Float(x1 + f64::from(*x2))),
        (DataType::Float(x1), DataType::Float(x2)) => Ok(DataType::Float(x1+x2)),
        _=> {
            return Err(
                ChapError::runtime_with_msg(0, "add function works only with numbers int and float".to_string())
            );
        }
    }
}

//TODO make this gloale
fn get_var(runtime: &Runtime, name: &str) -> Result<DataType>{

    match runtime.variables.get(name){
        Some(x) => Ok(x.clone()),
        None => return Err(
            ChapError::runtime_with_msg(0, "".to_string())
        ),
    }
}