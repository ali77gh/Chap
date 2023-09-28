use crate::{runtime::runtime::Runtime, common::{data_type::DataType, errors::ChapError, param::Param}};
use crate::common::errors::Result;


pub fn get_var(runtime: &Runtime, name: &str, line_number: u32) -> Result<DataType>{

    match runtime.variables.get(name){
        Some(x) => Ok(x.clone()),
        None => return Err(
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
        Param::Tag(_) => return Err(ChapError::runtime_with_msg(line_number, "can not convert a tag to datatype".to_string())),
        Param::Value(value) => return Ok(value),
        Param::Variable(name) => match runtime.variables.get(name){
            Some(x) => return Ok(x),
            None => return Err(
                ChapError::runtime_with_msg(line_number, format!("variable {} is not defind", name))
            ),
        },
    }
    
}
