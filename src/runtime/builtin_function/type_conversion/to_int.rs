use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn to_int(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let result = match p1 {
        DataType::String(s) => {
            match s.parse(){
                Ok(x) => DataType::Int(x),
                Err(_) => {
                    return Err(
                        ChapError::runtime_with_msg(executable.line_number, format!("can not parse {} to int", s))
                    );
                },
            }
        },
        DataType::Float(f) => {
            DataType::Int(*f as i32)
        },
        _ => {
            return Err(
                ChapError::runtime_with_msg(executable.line_number, format!("can not convert {} to int", p1.type_name()))
            );
        }
    };
    
    returns(runtime, executable, result)
}

