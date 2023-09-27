use crate::common::data_type::DataType;
use crate::common::param::Param;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};
use crate::common::errors::{Result, ChapError};


pub fn not_equal(runtime: &mut Runtime, executable: ExecutableLine)-> Result<()>{

    let mut r = match (executable.params.get(0) , executable.params.get(1)){
        (Some(Param::Value(DataType::Int(x1))), Some(Param::Value(DataType::Int(x2)))) => {
           x1==x2 
        },
        (Some(Param::Value(DataType::Float(x1))), Some(Param::Value(DataType::Float(x2)))) => {
           x1==x2 
        },
        (Some(Param::Variable(v1)), Some(Param::Variable(v2))) => {
            match (get_var(runtime, v1)?, get_var(runtime, v2)?) {
                (DataType::Int(x1), DataType::Int(x2)) => {
                    x1==x2
                },
                (DataType::Float(x1), DataType::Float(x2)) => {
                    x1==x2
                },
                _=>{
                    return Err(
                        ChapError::runtime_with_msg(executable.line_number, "equal function needs two param".to_string())
                    );
                }
            } 
        },
        _=>{
            return Err(
                ChapError::runtime_with_msg(executable.line_number, "equal function needs two param".to_string())
            );
        }
    };
    r = !r;
    match &executable.output_var{
        Some(x) => runtime.variables.insert(x.clone(), DataType::Bool(r)),
        None => return Err(
            ChapError::runtime_with_msg(executable.line_number, "".to_string())
        ),
    };
    
    Ok(())
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