use crate::runtime::builtin_function::utils::{param_to_datatype_mut, returns, param_to_datatype};
use crate::common::data_type::DataType;
use crate::{runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn remove_at(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    let p2 = param_to_datatype(runtime, executable.params.get(1), executable.line_number)?
        .clone();
    let p1 = param_to_datatype_mut(&mut (*runtime), executable.params.get(0), executable.line_number)?;

    let result=
    if let (DataType::List(list), DataType::Int(index)) = (p1, p2){
        if let Ok(index) = usize::try_from(index){
            if index==0 { 
                return Err(ChapError::runtime_with_msg(executable.line_number, "list index starts from 1".to_string()));
            }
            list.remove(index-1) // this is returning result
        }else{
            return Err(ChapError::runtime_with_msg(executable.line_number, format!("{} function negative index", executable.function_name)));
        }
    }else{
        return Err(ChapError::runtime_with_msg(executable.line_number, format!("correct form of {0} function: <list>, <index> -> {0} -> $var", executable.function_name)));
    };

    returns(runtime, executable, result)
}