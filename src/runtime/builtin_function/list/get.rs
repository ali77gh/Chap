use crate::runtime::builtin_function::utils::{param_to_datatype, returns};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn get(runtime: &mut Runtime, executable: &ExecutableLine) -> Result<()>{

    let p1 = param_to_datatype(runtime, executable.params.get(0), executable.line_number)?;
    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?;

    if let (DataType::List(list),DataType::Int(index)) = (p1,p2){
        if let Ok(x) = usize::try_from(*index){
            if x==0 { 
                return Err(ChapError::runtime_with_msg(executable.line_number, "list index starts from 1".to_string()));
            }
            let x = x - 1;
            let value = list.get(x);
            if let Some(value) = value{
                return returns(runtime, executable, value.clone());
            }else{
                return Err(ChapError::runtime_with_msg(executable.line_number, "index out of bound".to_string()));
            }
        }else{
            return Err(ChapError::runtime_with_msg(executable.line_number, "negative index".to_string()));
        }
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, "correct form of 'get' function: <list>, <index> -> get -> $item".to_string()));
    }
    
}