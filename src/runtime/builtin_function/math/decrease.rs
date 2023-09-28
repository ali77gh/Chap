use crate::common::data_type::DataType;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};
use crate::common::param::Param;

pub fn decrease(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    if let Some(Param::Variable(name)) = executable.params.get(0){
        match runtime.variables.get(name){
            Some(x) => match x {
                DataType::Int(x) => {
                    runtime.variables.insert(name.to_string(), DataType::Int(x-1));
                },
                _ => {
                    return Err(ChapError::runtime_with_msg(executable.line_number, format!("")));
                }
            },
            None => return Err(ChapError::runtime_with_msg(executable.line_number, format!("variable {} is not defind",name))),
        }
    }else {
        return Err(ChapError::runtime_with_msg(executable.line_number, "increase function need one variable".to_string()));
    }
    Ok(())
}
