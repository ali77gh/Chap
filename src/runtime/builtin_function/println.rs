use crate::common::errors::ChapError;
use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};

use crate::common::{
    errors::Result,
    param::Param,
    data_type::DataType,
};

pub fn println(runtime: &mut Runtime, executable: ExecutableLine)-> Result<()>{


    if executable.params.len() != 1{
        return Err(ChapError::runtime_with_msg(
            executable.line_number, "".to_string()
        ));
    }

    let msg: String = match executable.params.get(0).unwrap() { 
        Param::Tag(_) => return Err(
            ChapError::runtime_with_msg(executable.line_number, "tags cant be printed".to_string())
        ), 
        Param::Variable(var) => {
            match runtime.variables.get(var) {
                Some(data) => match data {
                    DataType::String(m) => m.clone(),
                    DataType::Int(i) => i.to_string(),
                    DataType::Float(f) => f.to_string(),
                    DataType::Bool(b) => b.to_string(),
                },
                None => return Err(
                    ChapError::runtime_with_msg(executable.line_number, format!("variable {} is undefind",var))
                )
            }
        }, 
        Param::Value(val) => match val {
            DataType::String(m) => m.clone(),
            DataType::Int(i) => i.to_string(),
            DataType::Float(f) => f.to_string(),
            DataType::Bool(b) => b.to_string(), 
        }, 
        
    };
    
    runtime.std_out(&msg);
    
    Ok(())
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn print_string(){
        let mut runtime = Runtime::new(|x|{
            assert_eq!(x, "test");
        }, ||{ "".to_string() });


        println(
            &mut runtime,
            ExecutableLine::new(1,
                "".to_string(),
                vec![Param::Value(DataType::String("test".to_string()))],
                None
            )
        ).unwrap();
    }

    #[test]
    fn print_int(){
        let mut runtime = Runtime::new(|x|{
            assert_eq!(x, "2");
        }, ||{ "".to_string() });

        println(
            &mut runtime,
            ExecutableLine::new(1,
                "".to_string(),
                vec![Param::Value(DataType::Int(2))],
                None
            )
        ).unwrap();
    }
}