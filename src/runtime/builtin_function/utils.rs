use crate::{runtime::Runtime, common::{data_type::DataType, errors::ChapError, param::Param, executable::ExecutableLine}};
use crate::common::errors::Result;

#[allow(dead_code)]
pub fn get_var(runtime: &Runtime, name: &str, line_number: u32) -> Result<DataType>{

    match runtime.variables.get(name){
        Some(x) => Ok(x.clone()),
        None => Err(
            ChapError::runtime_with_msg(line_number, format!("variable {} is not defind",name))
        ),
    }
}

pub fn param_to_datatype<'a>(runtime: &'a Runtime, param: Option<&'a Param>, line_number: u32) -> Result<&'a DataType>{

    let param = match param {
        Some(x) => x,
        None => return Err(ChapError::static_analyzer_with_msg(line_number, "function needs more params".to_string())),
    };

    match param {
        Param::Tag(_) => Err(ChapError::runtime_with_msg(line_number, "can not convert a tag to datatype".to_string())),
        Param::Value(value) => Ok(value),
        Param::Variable(name) => match runtime.variables.get(name){
            Some(x) => Ok(x),
            None => Err(
                ChapError::runtime_with_msg(line_number, format!("variable {} is not defind", name))
            ),
        },
    }
    
}

pub fn param_to_datatype_mut<'a>(runtime: &'a mut Runtime, param: Option<&'a Param>, line_number: u32) -> Result<&'a mut DataType>{

    let param = match param {
        Some(x) => x,
        None => return Err(ChapError::static_analyzer_with_msg(line_number, "function needs more params".to_string())),
    };

    match param {
        Param::Tag(_) => Err(ChapError::runtime_with_msg(line_number, "can not convert a tag to datatype".to_string())),
        Param::Value(_) => Err(ChapError::runtime_with_msg(line_number, "variable expected.".to_string())),
        Param::Variable(name) => match runtime.variables.get_mut(name){
            Some(x) => Ok(x),
            None => Err(
                ChapError::runtime_with_msg(line_number, format!("variable {} is not defind", name))
            ),
        },
    }
    
}

pub fn returns(runtime: &mut Runtime, executable: &ExecutableLine, result: DataType) -> Result<()>{

    if let Some(var_name) = &executable.output_var{
        runtime.variables.insert(var_name.clone(), result);
    }else{
        runtime.std_out(result.to_string().as_str());
    }
    Ok(())
}