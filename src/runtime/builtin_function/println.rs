use crate::{runtime::runtime::Runtime, common::executable::ExecutableLine};

use crate::common::{
    errors::Result,
    param::Param,
    data_type::DataType,
};

pub fn println(runtime: &mut Runtime, executable: &ExecutableLine)-> Result<()>{

    // TODO print multiple params in loop
    let msg: &str = match executable.params.get(0).unwrap() { 
        Param::Tag(tag) => todo!(), // TODO Err
        Param::Variable(var) => {
            match runtime.variables.get(var) {
                Some(data) => match data {
                    DataType::String(m) => m.as_str(),
                    DataType::Int(_) => todo!(),
                    DataType::Float(_) => todo!(),
                },
                None => todo!(),// TODO error
            }
        }, 
        Param::Value(val) => match val {
            DataType::String(m) => m.as_str(),
            DataType::Int(_) => todo!(), // TODO Convert 
            DataType::Float(_) => todo!(), // TODO Convert 
        }, 
        
    };
    
    runtime.std_out(msg);
    
    Ok(())
}